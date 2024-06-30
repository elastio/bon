mod error_handling;
mod normalization;

use darling::FromMeta;
use heck::AsPascalCase;
use itertools::Itertools;
use proc_macro2::TokenStream as TokenStream2;
use prox::prelude::*;
use prox::Result;
use quote::{quote, ToTokens};

pub(crate) use error_handling::error_into_token_stream;

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
    func: syn::ItemFn,
    setters: Vec<Setter>,
    builder_ident: syn::Ident,
    builder_private_impl_ident: syn::Ident,
}

impl MacroCtx {
    fn from_item_fn(func: syn::ItemFn) -> Result<Self> {
        let func = normalization::normalize_fn_item(func);
        let pascal_case_func = AsPascalCase(func.sig.ident.to_string());
        let builder_ident = quote::format_ident!("{pascal_case_func}Builder");
        let builder_private_impl_ident = quote::format_ident!("__{builder_ident}PrivateImpl");

        let setters: Vec<_> = func
            .sig
            .inputs
            .iter()
            .map(Setter::from_fn_arg)
            .try_collect()?;

        let ctx = MacroCtx {
            func,
            setters,
            builder_ident,
            builder_private_impl_ident,
        };

        Ok(ctx)
    }

    fn setter_idents(&self) -> impl Iterator<Item = syn::Ident> + '_ {
        self.setters
            .iter()
            .map(|setter| setter.fn_arg_ident.clone())
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

    fn func_ident(&self) -> syn::Ident {
        let ident = &self.func.sig.ident;
        quote::format_ident!("__positional_{}", ident.to_string())
    }

    fn func_generics_decl(&self) -> impl Iterator<Item = &syn::GenericParam> + '_ {
        self.func.sig.generics.params.iter()
    }

    fn func_generic_args(&self) -> impl Iterator<Item = syn::GenericArgument> + '_ {
        let params = &self.func.sig.generics.params;
        params.iter().map(|param| match param {
            syn::GenericParam::Lifetime(param) => {
                syn::GenericArgument::Lifetime(param.lifetime.clone())
            }
            syn::GenericParam::Type(param) => {
                let ident = &param.ident;
                syn::GenericArgument::Type(syn::parse_quote!(#ident))
            }
            syn::GenericParam::Const(param) => {
                let ident = &param.ident;
                syn::GenericArgument::Const(syn::parse_quote!(#ident))
            }
        })
    }

    fn normalized_func_where_clause(&self) -> Option<&syn::WhereClause> {
        self.func.sig.generics.where_clause.as_ref()
    }

    fn output(self) -> TokenStream2 {
        let entry_func = self.entry_func();
        let builder_decl = self.builder_decl();
        let call_method_impl = self.call_method_impl();
        let setter_methods_impls = self.setter_methods_impls();
        let positional_func = self.positional_func();

        quote! {
            #entry_func
            #builder_decl
            #call_method_impl
            #setter_methods_impls
            #positional_func
        }
    }

    fn positional_func(&self) -> TokenStream2 {
        // Change to an implementation function's visibility to private inside of a
        // child module to avoid exposing it to the surrounding code. The surrounding
        // code is supposed to use this function through the builder only.
        let mut positional_func = self.func.clone();
        positional_func.sig.ident = self.func_ident();

        normalization::strip_doc_comments_from_args(&mut positional_func.sig);

        // It's fine if there are too many positional arguments in the function
        // because the whole purpose of this macro is to fight with this problem
        // at the call site by generating a builder, while keeping the fn definition
        // site the same with tons of positional arguments which don't harm readability
        // there because their names are explicitly specified at the definition site.
        positional_func
            .attrs
            .push(syn::parse_quote!(#[allow(clippy::too_many_arguments)]));

        positional_func.into_token_stream()
    }

    fn entry_func(&self) -> TokenStream2 {
        let docs = self.func.attrs.iter().filter(|attr| {
            let syn::Meta::NameValue(attr) = &attr.meta else {
                return false;
            };
            attr.path.is_ident("doc")
        });

        let current_mod_vis = &self.func.vis;
        let builder_ident = &self.builder_ident;
        let builder_private_impl_ident = &self.builder_private_impl_ident;
        let entry_func_ident = &self.func.sig.ident;

        // TODO: we can use a shorter syntax with anonymous lifetimes to make
        // the generate code and function signature displayed by rust-analyzer
        // a bit shorter and easier to read. However, the caveat is that we can
        // do this only for lifetimes that have no bounds and if they don't appear
        // in the where clause. Research `darling`'s lifetime tracking API and
        // maybe implement this in the future

        let generics_decl = self.func_generics_decl();
        let generic_args = self.func_generic_args();
        let where_clause = self.normalized_func_where_clause();

        let setter_idents = self.setter_idents();
        let phantom_data = self.phantom_data_field_init();

        let unset_state_types = self.unset_state_types();

        quote! {
            #(#docs)*
            #current_mod_vis fn #entry_func_ident<#(#generics_decl),*>() -> #builder_ident<
                #(#generic_args,)*
                #(#unset_state_types,)*
            >
            #where_clause
            {
                #builder_ident {
                    __private_impl: #builder_private_impl_ident {
                        #phantom_data
                        #(
                            #setter_idents: ::core::default::Default::default(),
                        )*
                    }
                }
            }
        }
    }

    fn phantom_data(&self) -> Option<TokenStream2> {
        let func_generics = &self.func.sig.generics;
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
            ::core::marker::PhantomData<(
                #(#lifetime_refs,)*
                #(#type_refs,)*
            )>
        })
    }

    fn phantom_data_field_init(&self) -> Option<TokenStream2> {
        self.phantom_data().map(|_| {
            quote! {
                _phantom: ::core::marker::PhantomData,
            }
        })
    }

    fn builder_decl(&self) -> TokenStream2 {
        let vis = &self.func.vis;
        let builder_ident = &self.builder_ident;
        let builder_private_impl_ident = &self.builder_private_impl_ident;
        let setter_idents = self.setter_idents();
        let fields_states_vars = self.fields_states_vars().collect_vec();
        let generics_decl = self.func_generics_decl().collect_vec();
        let where_clause = self.normalized_func_where_clause();
        let generic_args = self.func_generic_args();

        let phantom_data = self.phantom_data().map(|phantom_data| {
            quote! {
                _phantom: #phantom_data,
            }
        });

        quote! {
            #vis struct #builder_ident<
                #(#generics_decl,)*

                // We could set default values for `fields_states_vars` here
                // to their initial unset states, but we don't do that and
                // pass these generic params explicitly to workaround the following
                // bug in rust-analyzer where it stops providing completions for
                // builder methods completely if we rely on default generic type
                // params values. See the issue in rust-analyzer for details:
                // https://github.com/rust-lang/rust-analyzer/issues/17515
                #(#fields_states_vars,)*
            >
            #where_clause
            {
                /// Please don't touch this field. It's an implementation
                /// detail that is exempt from the API stability guarantees.
                /// It's visible to you only because of the limitations of
                /// the Rust language.
                ///
                /// The limitation is that we can't make the fields of the
                /// generated struct private other than by placing its
                /// declaration inside of a nested submodule. However, we
                /// can't do that because this breaks support for fn items
                /// declared inside of other fn items like this:
                ///
                /// ```ignore
                /// use buildy::builder;
                ///
                /// fn foo() {
                ///     struct Foo;
                ///
                ///     #[builder]
                ///     fn nested(foo: Foo) {}
                ///
                ///     nested().foo(Foo).call();
                /// }
                /// ```
                ///
                /// If we were to generate a child module like this then code
                /// in that child module would lose access to the symbol `Foo`
                /// in the parent module. The following code doesn't compile.
                ///
                /// ```ignore
                /// fn foo() {
                ///     struct Foo;
                ///
                ///     mod __private_child_module {
                ///         use super::*;
                ///
                ///         pub(super) struct Builder {
                ///             foo: Foo,
                ///         }
                ///     }
                /// }
                /// ```
                ///
                /// `Foo` symbol is inaccessible inside of `__private_child_module`
                /// because it is defined inside of the function `foo()` and not
                /// inside of the parent module.
                ///
                /// Child modules are kinda implicitly "hoisted" to the top-level of
                /// the module and they can't see the local symbols defined inside
                /// of the same function scope.
                __private_impl: #builder_private_impl_ident<
                    #(#generic_args,)*
                    #(#fields_states_vars,)*
                >
            }

            /// This struct exists only to reduce the number of private fields
            /// that pop up in IDE completions for developers. It groups all
            /// the private fields in it leaving the builder type higher with
            /// just a single field of this type that documents the fact that
            /// the developers shouldn't touch it.
            struct #builder_private_impl_ident<
                #(#generics_decl,)*
                #(#fields_states_vars,)*
            >
            #where_clause
            {
                #phantom_data
                #( #setter_idents: #fields_states_vars, )*
            }
        }
    }

    fn call_method_impl(&self) -> TokenStream2 {
        let asyncness = &self.func.sig.asyncness;
        let maybe_await = asyncness.is_some().then(|| quote!(.await));

        let positional_func_ident = self.func_ident();
        let setter_idents = self.setter_idents();
        let unsafety = &self.func.sig.unsafety;
        let vis = &self.func.vis;
        let builder_ident = &self.builder_ident;
        let output_type = &self.func.sig.output;
        let fields_states_vars = self.fields_states_vars().collect_vec();
        let set_state_types = self.set_state_types();
        let generics_decl = self.func_generics_decl();
        let where_clause_predicates = self
            .normalized_func_where_clause()
            .into_iter()
            .flat_map(|where_clause| where_clause.predicates.iter());

        let generic_builder_args = self.func_generic_args();

        // Filter out lifetime generic arguments, because they are not needed
        // to be specified explicitly when calling the function. This also avoids
        // the problem that it's not always possible to specify lifetimes in
        // the turbofish syntax. See the problem of late-bound lifetimes specification
        // in the issue https://github.com/rust-lang/rust/issues/42868
        let generic_fn_args = self
            .func_generic_args()
            .filter(|arg| !matches!(arg, syn::GenericArgument::Lifetime(_)));

        quote! {
            impl<
                #(#generics_decl,)*
                #(#fields_states_vars),*
            >
            #builder_ident<
                #(#generic_builder_args,)*
                #(#fields_states_vars),*
            >
            where
                #( #where_clause_predicates, )*
                #(#fields_states_vars: std::convert::Into<#set_state_types>,)*
            {
                #vis #asyncness #unsafety fn call(self) #output_type {
                    #positional_func_ident::<#(#generic_fn_args,)*>(
                        #(
                            self.__private_impl.#setter_idents.into().into_inner()
                        ),*
                    )
                    #maybe_await
                }
            }
        }
    }

    fn setter_methods_impls(&self) -> TokenStream2 {
        let generic_args = self.func_generic_args().collect_vec();

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
                                quote!(buildy::Set::new(value))
                            } else {
                                let ident = &other_setter.fn_arg_ident;
                                quote!(self.__private_impl.#ident)
                            }
                        });

                let setter_ident = &setter.fn_arg_ident;
                let setter_type = &setter.fn_arg_type;
                let docs = &setter.docs;
                let vis = &self.func.vis;
                let builder_ident = &self.builder_ident;
                let builder_private_impl_ident = &self.builder_private_impl_ident;
                let setter_idents = self.setter_idents();
                let phantom_data = self.phantom_data_field_init();
                let generics_decl = self.func_generics_decl();
                let where_clause = self.normalized_func_where_clause();

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
                                __private_impl: #builder_private_impl_ident {
                                    #phantom_data
                                    #(
                                        #setter_idents: #field_exprs,
                                    )*
                                }
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
            generic_var_ident: quote::format_ident!("__B{}", pat.ident.to_pascal_case()),
            fn_arg_ident: pat.ident.clone(),
            fn_arg_type: arg.ty.clone(),
            docs,
        })
    }

    fn unset_state_type(&self) -> TokenStream2 {
        let ty = &self.fn_arg_type;
        ty.option_item_ty()
            .map(|ty| quote!(buildy::Optional<#ty>))
            .unwrap_or_else(|| quote!(buildy::Required<#ty>))
    }

    fn set_state_type(&self) -> TokenStream2 {
        let ty = &self.fn_arg_type;
        quote!(buildy::Set<#ty>)
    }
}

fn skip_nth<I: IntoIterator>(iterable: I, n: usize) -> impl Iterator<Item = I::Item> {
    iterable
        .into_iter()
        .enumerate()
        .filter(move |(index, _)| *index != n)
        .map(|(_, item)| item)
}
