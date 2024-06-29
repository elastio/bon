use darling::FromMeta;
use heck::AsPascalCase;
use itertools::Itertools;
use proc_macro2::TokenStream as TokenStream2;
use prox::prelude::*;
use prox::Result;
use quote::{quote, ToTokens};

#[derive(Debug, FromMeta)]
pub(crate) struct Opts {
    // There may be config options for the proc macro in the future here
}

pub(crate) fn generate(_: Opts, item: syn::Item) -> Result<TokenStream2> {
    match item {
        syn::Item::Fn(func) => {
            let ctx = MacroCtx::from_item_fn(func)?;
            Ok(ctx.output())
        }
        _ => prox::bail!(
            &item,
            "The attribute is expected to be placed on an `fn` \
            item, but it was placed on other syntax instead"
        ),
    }
}

struct MacroCtx {
    original_func: syn::ItemFn,
    setters: Vec<Setter>,
    builder_ident: syn::Ident,
    vis: syn::Visibility,
}

impl MacroCtx {
    fn from_item_fn(original_func: syn::ItemFn) -> Result<Self> {
        let pascal_case_func = AsPascalCase(original_func.sig.ident.to_string());
        let builder_ident = quote::format_ident!("{pascal_case_func}Builder");

        let setters: Vec<_> = original_func
            .sig
            .inputs
            .iter()
            .map(Setter::from_fn_arg)
            .try_collect()?;

        let vis = original_func
            .vis
            .clone()
            .into_equivalent_in_child_module()?;

        let ctx = MacroCtx {
            vis,
            original_func,
            setters,
            builder_ident,
        };

        Ok(ctx)
    }

    fn setter_idents(&self) -> impl Iterator<Item = syn::Ident> + '_ {
        self.setters
            .iter()
            .map(|setter| setter.fn_arg_ident.to_snake_case())
    }

    fn unset_state_types(&self) -> impl Iterator<Item = TokenStream2> + '_ {
        self.setters.iter().map(|arg| arg.unset_state_type())
    }

    fn set_state_types(&self) -> impl Iterator<Item = TokenStream2> + '_ {
        self.setters.iter().map(|arg| arg.set_state_type())
    }

    fn fields_states_vars(&self) -> impl Iterator<Item = &syn::Ident> + '_ {
        self.setters.iter().map(|arg| &arg.generic_var_ident)
    }

    fn positional_func_ident(&self) -> syn::Ident {
        let ident = &self.original_func.sig.ident;
        quote::format_ident!("__buildy_func_{}", ident.to_string())
    }

    fn impl_mod_ident(&self) -> syn::Ident {
        quote::format_ident!(
            "__buildy_mod_{}",
            &self.original_func.sig.ident.to_snake_case()
        )
    }

    fn original_func_generics_decl(&self) -> impl Iterator<Item = &syn::GenericParam> + '_ {
        self.original_func.sig.generics.params.iter()
    }

    fn original_func_generic_args(&self) -> impl Iterator<Item = TokenStream2> + '_ {
        let params = &self.original_func.sig.generics.params;
        params.iter().map(|param| match param {
            syn::GenericParam::Lifetime(param) => param.lifetime.to_token_stream(),
            syn::GenericParam::Type(param) => param.ident.to_token_stream(),
            syn::GenericParam::Const(param) => param.ident.to_token_stream(),
        })
    }

    fn original_func_where_clause(&self) -> Option<&syn::WhereClause> {
        self.original_func.sig.generics.where_clause.as_ref()
    }

    fn output(self) -> TokenStream2 {
        let impl_mod_ident = self.impl_mod_ident();
        let entry_func = self.entry_func();
        let builder_type_alias = self.builder_type_alias();
        let builder_declaration = self.builder_declaration();
        let builder_constructor = self.builder_constructor();
        let call_method_impl = self.call_method_impl();
        let setter_methods_impls = self.setter_methods_impls();
        let positional_func = self.positional_func();

        quote! {
            #builder_type_alias
            #entry_func

            #[doc(hidden)]
            mod #impl_mod_ident {
                use super::*;

                #builder_declaration
                #builder_constructor
                #call_method_impl
                #setter_methods_impls
            }

            #[doc(hidden)]
            #positional_func
        }
    }

    fn builder_type_alias(&self) -> TokenStream2 {
        let current_mod_vis = &self.original_func.vis;
        let builder_ident = &self.builder_ident;
        let unset_state_types = self.unset_state_types();
        let impl_mod_ident = self.impl_mod_ident();

        let generics_decl = self.original_func_generics_decl();
        let generic_args = self.original_func_generic_args();
        let where_clause = self.original_func_where_clause();

        quote! {
            // This lint is ignored, because bounds in type aliases are still useful
            // to make the following example usage compile:
            // ```
            // type Bar<T: IntoIterator> = T::Item;
            // ```
            // In this case the bound is necessary for `T::Item` access to be valid.
            // The compiler proposes this:
            //
            // > use fully disambiguated paths (i.e., `<T as Trait>::Assoc`) to refer
            // > to associated types in type aliases
            //
            // But, come on... Why would you want to write `T::Item` as `<T as IntoIterator>::Item`
            // especially if that `T::Item` access is used in multiple places? It's a waste of time
            // to implement logic that rewrites the user's type expressions to that syntax when just
            // having bounds on the type alias is enough already.
            #[allow(type_alias_bounds)]
            #current_mod_vis type #builder_ident<#(#generics_decl),*>
            // The where clause in this position will be deprecated. The preferred
            // position will be at the end of the entire type alias syntax construct.
            // See details at https://github.com/rust-lang/rust/issues/112792.
            //
            // However, at the time of this writing the only way to put the where
            // clause on a type alias is here.
            //
            #where_clause
            = #impl_mod_ident::#builder_ident<
                #(#generic_args,)*
                #(#unset_state_types),*
            >;
        }
    }

    fn positional_func(&self) -> TokenStream2 {
        // Change to an implementation function's visibility to private inside of a
        // child module to avoid exposing it to the surrounding code. The surrounding
        // code is supposed to use this function through the builder only.
        let mut positional_func = self.original_func.clone();
        positional_func.sig.ident = self.positional_func_ident();

        // Remove all doc comments attributes from function arguments, because they are
        // not valid in that position in regular Rust code. The cool trick is that they
        // are still valid syntactically when a proc macro like this one pre-processes
        // them and removes them from the expanded code. We use the doc comments to put
        // them on the generated setter methods.
        for arg in &mut positional_func.sig.inputs {
            arg.attrs_mut().retain(|attr| !attr.is_doc());
        }

        positional_func.into_token_stream()
    }

    fn entry_func(&self) -> TokenStream2 {
        let docs = self.original_func.attrs.iter().filter(|attr| {
            let syn::Meta::NameValue(attr) = &attr.meta else {
                return false;
            };
            attr.path.is_ident("doc")
        });

        let current_mod_vis = &self.original_func.vis;
        let builder_ident = &self.builder_ident;
        let entry_func_ident = &self.original_func.sig.ident;

        let generics_decl = self.original_func_generics_decl();
        let generic_args = self.original_func_generic_args();
        let where_clause = self.original_func_where_clause();

        quote! {
            #(#docs)*
            #current_mod_vis fn #entry_func_ident<#(#generics_decl),*> ()
                -> #builder_ident<#(#generic_args),*>
            #where_clause
            {
                #builder_ident::new()
            }
        }
    }

    fn phantom_data(&self) -> Option<TokenStream2> {
        let func_generics = &self.original_func.sig.generics;
        let generic_lifetimes = func_generics.lifetimes().collect_vec();
        let generic_type_params = func_generics.type_params().collect_vec();

        if generic_type_params.is_empty() && generic_lifetimes.is_empty() {
            return None;
        }

        let lifetime_refs = generic_lifetimes.iter().map(|lifetime| {
            let lifetime = &lifetime.lifetime;
            quote!(&#lifetime ())
        });

        let type_refs = generic_type_params
            .iter()
            .map(|type_param| &type_param.ident);

        Some(quote! {
            ::std::marker::PhantomData<(
                #(#lifetime_refs,)*
                #(#type_refs,)*
            )>
        })
    }

    fn phantom_data_field_init(&self) -> Option<TokenStream2> {
        self.phantom_data().map(|_| {
            quote! {
                _phantom: ::std::marker::PhantomData,
            }
        })
    }

    fn builder_declaration(&self) -> TokenStream2 {
        let vis = &self.vis;
        let builder_ident = &self.builder_ident;
        let setter_idents = self.setter_idents();
        let fields_states_vars = self.fields_states_vars().collect_vec();

        let generics_decl = self.original_func_generics_decl();
        let where_clause = self.original_func_where_clause();

        let phantom_data = self.phantom_data().map(|phantom_data| {
            quote! {
                _phantom: #phantom_data,
            }
        });

        quote! {
            #vis struct #builder_ident<#(#generics_decl,)* #(#fields_states_vars,)*>
            #where_clause
            {
                #phantom_data
                #( #setter_idents: #fields_states_vars, )*
            }
        }
    }

    fn builder_constructor(&self) -> TokenStream2 {
        let vis = &self.vis;
        let builder_ident = &self.builder_ident;
        let setter_idents = self.setter_idents();
        let unset_state_types = self.unset_state_types();
        let phantom_data = self.phantom_data_field_init();

        let generics_decl = self.original_func_generics_decl();
        let generic_args = self.original_func_generic_args();
        let where_clause = self.original_func_where_clause();

        quote! {
            impl<#(#generics_decl),*> #builder_ident<
                #(#generic_args,)*
                #(#unset_state_types),*
            >
            #where_clause
            {
                #vis fn new() -> Self {
                    Self {
                        #phantom_data
                        #(
                            #setter_idents: ::std::default::Default::default(),
                        )*
                    }
                }
            }
        }
    }

    fn call_method_impl(&self) -> TokenStream2 {
        let asyncness = &self.original_func.sig.asyncness;
        let maybe_await = asyncness.is_some().then(|| quote!(.await));

        let positional_func_ident = self.positional_func_ident();
        let setter_idents = self.setter_idents();
        let unsafety = &self.original_func.sig.unsafety;
        let vis = &self.vis;
        let builder_ident = &self.builder_ident;
        let output_type = &self.original_func.sig.output;
        let fields_states_vars = self.fields_states_vars().collect_vec();
        let set_state_types = self.set_state_types();
        let generics_decl = self.original_func_generics_decl();
        let generic_args = self.original_func_generic_args().collect_vec();
        let where_clause_predicates = self
            .original_func_where_clause()
            .into_iter()
            .flat_map(|where_clause| where_clause.predicates.iter());

        quote! {
            impl<
                #(#generics_decl,)*
                #(#fields_states_vars),*
            >
            #builder_ident<
                #(#generic_args,)*
                #(#fields_states_vars),*
            >
            where
                #( #where_clause_predicates, )*
                #(#fields_states_vars: std::convert::Into<#set_state_types>,)*
            {
                #vis #asyncness #unsafety fn call(self) #output_type {
                    #positional_func_ident::<#(#generic_args,)*>(
                        #(
                            self.#setter_idents.into().into_inner()
                        ),*
                    )
                    #maybe_await
                }
            }
        }
    }

    fn setter_methods_impls(&self) -> TokenStream2 {
        let generic_args = self.original_func_generic_args().collect_vec();

        self.setters
            .iter()
            .enumerate()
            .map(|(setter_index, setter)| {
                let fields_states_vars = skip_nth(self.fields_states_vars(), setter_index);

                let input_builder_fields_states =
                    self.setters
                        .iter()
                        .enumerate()
                        .map(|(other_setter_index, other_setter)| {
                            if other_setter_index == setter_index {
                                other_setter.unset_state_type()
                            } else {
                                other_setter.generic_var_ident.to_token_stream()
                            }
                        });

                let output_builder_fields_states =
                    self.setters
                        .iter()
                        .enumerate()
                        .map(|(other_setter_index, other_setter)| {
                            if other_setter_index == setter_index {
                                other_setter.set_state_type()
                            } else {
                                other_setter.generic_var_ident.to_token_stream()
                            }
                        });

                let field_exprs =
                    self.setters
                        .iter()
                        .enumerate()
                        .map(|(other_setter_index, other_setter)| {
                            if other_setter_index == setter_index {
                                quote!(::buildy::Set::new(value))
                            } else {
                                let ident = &other_setter.fn_arg_ident;
                                quote!(self.#ident)
                            }
                        });

                let setter_ident = &setter.fn_arg_ident;
                let setter_type = &setter.fn_arg_type;
                let docs = &setter.docs;
                let vis = &self.vis;
                let builder_ident = &self.builder_ident;
                let setter_idents = self.setter_idents();
                let phantom_data = self.phantom_data_field_init();
                let generics_decl = self.original_func_generics_decl();
                let where_clause = self.original_func_where_clause();

                quote! {
                    impl<
                        #(#generics_decl,)*
                        #(#fields_states_vars),*
                    >
                    #builder_ident<
                        #(#generic_args,)*
                        #(#input_builder_fields_states),*
                    >
                    #where_clause
                    {
                        #(#docs)*
                        #vis fn #setter_ident(self, value: #setter_type) -> #builder_ident<
                            #(#generic_args,)*
                            #(#output_builder_fields_states,)*
                        >
                        {
                            #builder_ident {
                                #phantom_data
                                #(
                                    #setter_idents: #field_exprs,
                                )*
                            }
                        }
                    }
                }
            })
            .collect()
    }
}

struct Setter {
    /// Original name of the argument in the function signature is used as the
    /// name of the builder field and in its setter methods. Function parameters
    /// conventionally use snake_case in Rust, but this isn't enforced, so this
    /// field isn't guaranteed to be in snake case, but 99% of the time it will be.
    fn_arg_ident: syn::Ident,

    /// Doc comments for the setter methods are copied from the doc comments placed
    /// on top of individual arguments in the original function. Yes, doc comments
    /// are not valid on function arguments in regular Rust, but they are valid if
    /// a proc macro like this one pre-processes them and removes them from the
    /// expanded code.
    docs: Vec<syn::Attribute>,

    /// Type of the function argument that corresponds to this field. This is the
    /// resulting type that the builder should generate a setter for.
    fn_arg_type: Box<syn::Type>,

    /// Derived from [`BuilderField::ident`] to make a conventional PascalCase
    /// generic type parameter identifier for this field.
    generic_var_ident: syn::Ident,
}

impl Setter {
    fn from_fn_arg(arg: &syn::FnArg) -> Result<Self> {
        let arg = match arg {
            syn::FnArg::Receiver(_) => {
                prox::bail!(arg, "Methods with `self` aren't supported yet")
            }
            syn::FnArg::Typed(arg) => arg,
        };

        let syn::Pat::Ident(pat) = arg.pat.as_ref() else {
            // We may allow setting a name for the builder method in parameter
            // attributes and relax this requirement
            prox::bail!(
                &arg.pat,
                "Only simple identifiers in function arguments supported \
                to infer the name of builder methods"
            );
        };

        let docs = arg
            .attrs
            .iter()
            .filter(|attr| attr.is_doc())
            .cloned()
            .collect();

        Ok(Self {
            generic_var_ident: quote::format_ident!("__Buildy{}", pat.ident.to_pascal_case()),
            fn_arg_ident: pat.ident.clone(),
            fn_arg_type: arg.ty.clone(),
            docs,
        })
    }

    fn unset_state_type(&self) -> TokenStream2 {
        let ty = &self.fn_arg_type;
        ty.option_item_ty()
            .map(|ty| quote!(::buildy::Optional<#ty>))
            .unwrap_or_else(|| quote!(::buildy::Required<#ty>))
    }

    fn set_state_type(&self) -> TokenStream2 {
        let ty = &self.fn_arg_type;
        quote!(::buildy::Set<#ty>)
    }
}

fn skip_nth<I: IntoIterator>(iterable: I, n: usize) -> impl Iterator<Item = I::Item> {
    iterable
        .into_iter()
        .enumerate()
        .filter(move |(index, _)| *index != n)
        .map(|(_, item)| item)
}
