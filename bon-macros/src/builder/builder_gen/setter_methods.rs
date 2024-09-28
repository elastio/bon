use super::{BuilderGenCtx, NamedMember};
use crate::util::prelude::*;
use quote::quote;

pub(crate) struct MemberSettersCtx<'a> {
    builder_gen: &'a BuilderGenCtx,
    member: &'a NamedMember,
}

impl<'a> MemberSettersCtx<'a> {
    pub(crate) fn new(builder_gen: &'a BuilderGenCtx, member: &'a NamedMember) -> Self {
        Self {
            builder_gen,
            member,
        }
    }

    pub(crate) fn setter_methods(&self) -> TokenStream2 {
        let member_type = self.member.norm_ty.as_ref();

        if let Some(inner_type) = self.member.as_optional_norm_ty() {
            return self.setters_for_optional_member(inner_type);
        }

        let has_into = self.member.params.into.is_present();

        let (fn_param_type, maybe_into_call) = if has_into {
            (quote!(impl Into<#member_type>), quote!(.into()))
        } else {
            (quote!(#member_type), quote!())
        };

        self.setter_method(MemberSetterMethod {
            method_name: self.member.public_ident().clone(),
            fn_params: quote!(value: #fn_param_type),
            overwrite_docs: None,
            body: SetterBody::Default {
                member_init: quote!(Some(value #maybe_into_call)),
            },
        })
    }

    fn setters_for_optional_member(&self, inner_type: &syn::Type) -> TokenStream2 {
        let has_into = self.member.params.into.is_present();
        let (inner_type, maybe_map_conv_call) = if has_into {
            (quote!(impl Into<#inner_type>), quote!(.map(Into::into)))
        } else {
            (quote!(#inner_type), quote!())
        };

        let setter_method_name = self.member.public_ident().clone();

        // Preserve the original identifier span to make IDE's "go to definition" work correctly
        let option_method_name = syn::Ident::new(
            &format!("maybe_{}", setter_method_name.raw_name()),
            setter_method_name.span(),
        );

        // Option-less setter is just a shortcut for wrapping the value in `Some`.
        let optionless_setter_body = quote! {
            self.#option_method_name(Some(value))
        };

        let methods = [
            MemberSetterMethod {
                method_name: option_method_name,
                fn_params: quote!(value: Option<#inner_type>),
                overwrite_docs: Some(format!(
                    "Same as [`Self::{setter_method_name}`], but accepts \
                    an `Option` as input. See that method's documentation for \
                    more details.",
                )),
                body: SetterBody::Default {
                    member_init: quote!(value #maybe_map_conv_call),
                },
            },
            // We intentionally keep the name and signature of the setter method
            // for an optional member that accepts the value under the option the
            // same as the setter method for the required member to keep the API
            // of the builder compatible when a required member becomes optional.
            // To be able to explicitly pass an `Option` value to the setter method
            // users need to use the `maybe_{member_ident}` method.
            MemberSetterMethod {
                method_name: setter_method_name,
                fn_params: quote!(value: #inner_type),
                overwrite_docs: None,
                body: SetterBody::Custom(optionless_setter_body),
            },
        ];

        methods
            .into_iter()
            .map(|method| self.setter_method(method))
            .concat()
    }

    fn setter_method(&self, method: MemberSetterMethod) -> TokenStream2 {
        let MemberSetterMethod {
            method_name,
            fn_params,
            overwrite_docs,
            body,
        } = method;

        let docs = match overwrite_docs {
            Some(docs) => vec![syn::parse_quote!(#[doc = #docs])],
            None if !self.member.docs.is_empty() => self.member.docs.clone(),
            None => self.generate_docs_for_setter(),
        };

        let body = match body {
            SetterBody::Custom(body) => body,
            SetterBody::Default { member_init } => {
                let index = &self.member.index;

                let state_transition_call = if self.member.is_stateful() {
                    quote! {
                        .__private_transition_type_state()
                    }
                } else {
                    quote! {}
                };

                quote! {
                    self.__private_named_members.#index = #member_init;
                    self #state_transition_call
                }
            }
        };

        let member_pascal = &self.member.norm_ident_pascal;

        let state_transition =
            quote::format_ident!("Set{}", self.member.norm_ident_pascal.raw_name());

        let state_mod = &self.builder_gen.state_mod.ident;
        let generic_param = if self.builder_gen.stateful_members().take(2).count() == 1 {
            quote!()
        } else {
            quote!(<BuilderState>)
        };

        let state_transition = quote! {
            #state_mod::#state_transition #generic_param
        };

        let builder_ident = &self.builder_gen.builder_type.ident;
        let generic_args = &self.builder_gen.generics.args;

        let return_type = if self.member.is_stateful() {
            quote! {
                #builder_ident<#(#generic_args,)* #state_transition>
            }
        } else {
            quote! { Self }
        };

        let where_clause = if self.member.is_stateful() && !self.member.params.overwritable.is_present()
        {
            quote! {
                where
                    BuilderState::#member_pascal: #state_mod::IsUnset,
            }
        } else {
            quote! {}
        };


        let vis = &self.builder_gen.builder_type.vis;

        quote! {
            #( #docs )*
            #[allow(
                // This is intentional. We want the builder syntax to compile away
                clippy::inline_always,
                // We don't want to avoid using `impl Trait` in the setter. This way
                // the setter signature is easier to read, and anyway if you want to
                // specify a type hint for the method that accepts an `impl Into`, then
                // your design of this setter already went wrong.
                clippy::impl_trait_in_params
            )]
            #[inline(always)]
            #vis fn #method_name(mut self, #fn_params) -> #return_type
            #where_clause
            {
                #body
            }
        }
    }

    fn generate_docs_for_setter(&self) -> Vec<syn::Attribute> {
        let setter_core_name = self.member.public_ident();
        let start_fn_ident = &self.builder_gen.start_fn.ident;

        let more = |start_fn_path: &std::fmt::Arguments<'_>| {
            format!(" See {start_fn_path} for more info.")
        };

        let suffix = self
            .builder_gen
            .assoc_method_ctx
            .as_ref()
            .map(|assoc_ctx| {
                let ty_path = match assoc_ctx.self_ty.as_path() {
                    Some(ty_path) => ty_path,

                    // The type is quite complex. It's hard to generate a workable
                    // intra-doc link for it. So in order to avoid the broken
                    // intra-doc links lint we'll just skip adding more info.
                    _ => return String::new(),
                };

                let prefix = darling::util::path_to_string(&ty_path.path);
                more(&format_args!("[`{prefix}::{start_fn_ident}()`]"))
            })
            .unwrap_or_else(|| more(&format_args!("[`{start_fn_ident}()`]")));

        let docs = format!("Sets the value of `{setter_core_name}`.{suffix}");

        vec![syn::parse_quote!(#[doc = #docs])]
    }
}

enum SetterBody {
    Custom(TokenStream2),
    Default { member_init: TokenStream2 },
}

struct MemberSetterMethod {
    method_name: syn::Ident,
    fn_params: TokenStream2,
    overwrite_docs: Option<String>,
    body: SetterBody,
}
