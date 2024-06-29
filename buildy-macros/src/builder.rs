use darling::usage::GenericsExt;
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
        quote::format_ident!("__buildy_{}", ident.to_string())
    }

    fn impl_mod_ident(&self) -> syn::Ident {
        quote::format_ident!(
            "__buildy_{}_builder",
            &self.original_func.sig.ident.to_snake_case()
        )
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

        quote! {
            #current_mod_vis type #builder_ident = #impl_mod_ident::#builder_ident<
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

        quote! {
            #(#docs)*
            #current_mod_vis fn #entry_func_ident() -> #builder_ident {
                #builder_ident::new()
            }
        }
    }

    fn builder_declaration(&self) -> TokenStream2 {
        let vis = &self.vis;
        let builder_ident = &self.builder_ident;
        let setter_idents = self.setter_idents();
        let fields_states_vars = self.fields_states_vars().collect_vec();

        let func_generics = &self.original_func.sig.generics;
        let generic_params = func_generics.params.iter();
        let where_clause = &func_generics.where_clause;

        let generic_lifetimes = func_generics.lifetimes().collect_vec();
        let generic_type_params = func_generics.type_params().collect_vec();

        let phantom_data =
            (!generic_type_params.is_empty() || !generic_lifetimes.is_empty()).then(|| {
                let lifetime_refs = generic_lifetimes.iter().map(|lifetime| {
                    let lifetime = &lifetime.lifetime;
                    quote!(&#lifetime ())
                });

                let type_refs = generic_type_params
                    .iter()
                    .map(|type_param| &type_param.ident);

                quote! {
                    _phantom: ::std::marker::PhantomData<(
                        #(#lifetime_refs,)*
                        #(#type_refs,)*
                    )>,
                }
            });

        quote! {
            #vis struct #builder_ident<#(#generic_params,)* #(#fields_states_vars,)*>
            #where_clause
            {
                #phantom_data
                #(
                    #setter_idents: #fields_states_vars,
                )*
            }
        }
    }

    fn builder_constructor(&self) -> TokenStream2 {
        let vis = &self.vis;
        let builder_ident = &self.builder_ident;
        let setter_idents = self.setter_idents();
        let unset_state_types = self.unset_state_types();

        quote! {
            impl #builder_ident<#(#unset_state_types),*> {
                #vis fn new() -> Self {
                    Self {
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
        let generic_vars = self.fields_states_vars().collect_vec();
        let set_state_types = self.set_state_types();

        quote! {
            impl<#(#generic_vars),*> #builder_ident<#(#generic_vars),*>
            where
                #(#generic_vars: std::convert::Into<#set_state_types>,)*
            {
                #vis #asyncness #unsafety fn call(self) #output_type {
                    #positional_func_ident(
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
        self.setters
            .iter()
            .enumerate()
            .map(|(setter_index, setter)| {
                let generic_vars = skip_nth(self.fields_states_vars(), setter_index);

                let input_builder_generics_types =
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

                let output_builder_generic_types =
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
                let vis = &self.vis;
                let builder_ident = &self.builder_ident;
                let setter_idents = self.setter_idents();

                quote! {
                    impl<#(#generic_vars),*> #builder_ident<#(#input_builder_generics_types),*> {
                        #vis fn #setter_ident(self, value: #setter_type)
                            -> #builder_ident<#(#output_builder_generic_types),*>
                        {
                            #builder_ident {
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

        Ok(Self {
            generic_var_ident: pat.ident.to_pascal_case(),
            fn_arg_ident: pat.ident.clone(),
            fn_arg_type: arg.ty.clone(),
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
