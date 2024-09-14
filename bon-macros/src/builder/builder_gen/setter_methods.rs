use super::{BuilderGenCtx, NamedMember};
use crate::util::prelude::*;
use quote::quote;

/// Specifies the return type of the setter method. It is conditioned by the
/// `cfg(doc)`. If `cfg(doc)` is enabled, we want to generate a shorter type
/// signature that doesn't clutter the docs.
///
/// However, such type signature uses an associated type of a trait which makes
/// it much slower to compile when the code is built outside of `rustdoc`.
///
/// So the `doc_false` variant is used as an easier to compile alternative,
/// but still the equivalent of the same return type.
pub(crate) struct SettersReturnType {
    pub(crate) doc_true: TokenStream2,
    pub(crate) doc_false: TokenStream2,
}

pub(crate) struct MemberSettersCtx<'a> {
    builder_gen: &'a BuilderGenCtx,
    member: &'a NamedMember,
    return_type: SettersReturnType,
}

impl<'a> MemberSettersCtx<'a> {
    pub(crate) fn new(
        builder_gen: &'a BuilderGenCtx,
        member: &'a NamedMember,
        return_type: SettersReturnType,
    ) -> Self {
        Self {
            builder_gen,
            member,
            return_type,
        }
    }

    pub(crate) fn setter_methods(&self) -> Result<TokenStream2> {
        let member_type = self.member.norm_ty.as_ref();

        if let Some(inner_type) = self.member.as_optional_norm_ty() {
            return self.setters_for_optional_member(inner_type);
        }

        let has_into = self
            .member
            .param_into(&self.builder_gen.conditional_params)?;

        let (fn_param_type, maybe_into_call) = if has_into {
            (quote!(impl Into<#member_type>), quote!(.into()))
        } else {
            (quote!(#member_type), quote!())
        };

        Ok(self.setter_method(MemberSetterMethod {
            method_name: self.member.setter_method_core_name().clone(),
            fn_params: quote!(value: #fn_param_type),
            overwrite_docs: None,
            body: SetterBody::Default {
                member_init: quote!(::bon::private::Set(value #maybe_into_call)),
            },
        }))
    }

    fn setters_for_optional_member(&self, inner_type: &syn::Type) -> Result<TokenStream2> {
        let has_into = self
            .member
            .param_into(&self.builder_gen.conditional_params)?;
        let (inner_type, maybe_map_conv_call) = if has_into {
            (quote!(impl Into<#inner_type>), quote!(.map(Into::into)))
        } else {
            (quote!(#inner_type), quote!())
        };

        let setter_method_name = self.member.setter_method_core_name().clone();

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
                    member_init: quote!(::bon::private::Set(value #maybe_map_conv_call)),
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

        Ok(methods
            .into_iter()
            .map(|method| self.setter_method(method))
            .collect())
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

        let vis = &self.builder_gen.vis;

        let body = match body {
            SetterBody::Custom(body) => body,
            SetterBody::Default { member_init } => {
                let maybe_receiver_field = self
                    .builder_gen
                    .receiver()
                    .map(|_| quote!(__private_receiver: self.__private_receiver,));

                let maybe_start_fn_args_field = self
                    .builder_gen
                    .start_fn_args()
                    .next()
                    .map(|_| quote!(__private_start_fn_args: self.__private_start_fn_args,));

                let builder_ident = &self.builder_gen.builder_ident;

                let member_exprs = self.builder_gen.named_members().map(|other_member| {
                    if other_member.norm_ident == self.member.norm_ident {
                        return member_init.clone();
                    }
                    let index = &other_member.index;
                    quote!(self.__private_named_members.#index)
                });

                quote! {
                    #builder_ident {
                        __private_phantom: ::core::marker::PhantomData,
                        #maybe_receiver_field
                        #maybe_start_fn_args_field
                        __private_named_members: (#( #member_exprs, )*)
                    }
                }
            }
        };

        let member_state_type = &self.member.generic_var_ident;
        let SettersReturnType {
            doc_true: ret_doc_true,
            doc_false: ret_doc_false,
        } = &self.return_type;

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
            // The `cfg_attr` condition is for `doc`, so we don't pay the price
            // if invoking the `__return_type` macro in the usual case when the
            // code is compiled outside of `rustdoc`.
            #[cfg_attr(doc, bon::__return_type(#ret_doc_true))]
            #vis fn #method_name(self, #fn_params) -> #ret_doc_false
            where
                #member_state_type: ::bon::private::IsUnset,
            {
                #body
            }
        }
    }

    fn generate_docs_for_setter(&self) -> Vec<syn::Attribute> {
        let setter_core_name = self.member.setter_method_core_name();
        let start_fn_ident = &self.builder_gen.start_func.ident;

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
