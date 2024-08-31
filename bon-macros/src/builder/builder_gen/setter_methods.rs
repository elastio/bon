use super::{BuilderGenCtx, RegularMember};
use crate::util::prelude::*;
use quote::{quote, ToTokens};

impl BuilderGenCtx {
    pub(crate) fn setter_methods_impls_for_member(
        &self,
        member: &RegularMember,
    ) -> Result<TokenStream2> {
        let output_members_states = self.regular_members().map(|other_member| {
            if other_member.ident == member.ident {
                member.set_state_type().to_token_stream()
            } else {
                other_member.generic_var_ident.to_token_stream()
            }
        });

        let builder_ident = &self.builder_ident;
        let generics_decl = &self.generics.params;
        let generic_args: Vec<_> = self.generic_args().collect();
        let where_clause_predicates: Vec<_> = self
            .generics
            .where_clause
            .as_ref()
            .into_iter()
            .flat_map(|where_clause| &where_clause.predicates)
            .collect();

        let member_state_vars = self
            .regular_members()
            .filter(|other_member| other_member.ident != member.ident)
            .map(|other_member| &other_member.generic_var_ident);

        let input_member_states = self.regular_members().map(|other_member| {
            if other_member.ident == member.ident {
                quote!(::bon::private::Unset)
            } else {
                other_member.generic_var_ident.to_token_stream()
            }
        });

        let setter_methods = MemberSettersCtx::new(
            self,
            member,
            quote! {
                #builder_ident<
                    #(#generic_args,)*
                    (#(#output_members_states,)*)
                >
            },
        )
        .setter_methods()?;

        let allows = super::allow_warnings_on_member_types();

        Ok(quote! {
            #allows
            impl<
                #(#generics_decl,)*
                #(#member_state_vars,)*
            >
            #builder_ident<
                #(#generic_args,)*
                (#(#input_member_states,)*)
            >
            where
                #(#where_clause_predicates,)*
            {
                #setter_methods
            }
        })
    }
}

struct MemberSettersCtx<'a> {
    builder_gen: &'a BuilderGenCtx,
    member: &'a RegularMember,
    return_type: TokenStream2,
    norm_member_ident: syn::Ident,
}

impl<'a> MemberSettersCtx<'a> {
    fn new(
        builder_gen: &'a BuilderGenCtx,
        member: &'a RegularMember,
        return_type: TokenStream2,
    ) -> Self {
        let member_ident = &member.ident.to_string();
        let norm_member_ident = member_ident
            // Remove the leading underscore from the member name since it's used
            // to denote unused symbols in Rust. That doesn't mean the builder
            // API should expose that knowledge to the caller.
            .strip_prefix('_')
            .unwrap_or(member_ident);

        // Preserve the original identifier span to make IDE go to definition correctly
        // and make error messages point to the correct place.
        let norm_member_ident = syn::Ident::new_maybe_raw(norm_member_ident, member.ident.span());

        Self {
            builder_gen,
            member,
            return_type,
            norm_member_ident,
        }
    }

    fn setter_method_core_name(&self) -> syn::Ident {
        self.member
            .params
            .name
            .clone()
            .unwrap_or_else(|| self.norm_member_ident.clone())
    }

    fn setter_methods(&self) -> Result<TokenStream2> {
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
            method_name: self.setter_method_core_name(),
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

        let setter_method_name = self.setter_method_core_name();

        // Preserve the original identifier span to make IDE go to definition correctly
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
        let return_type = &self.return_type;
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
                    .assoc_method_ctx
                    .as_ref()
                    .is_some_and(|ctx| ctx.receiver.is_some())
                    .then(|| quote!(__private_receiver: self.__private_receiver,));

                let builder_ident = &self.builder_gen.builder_ident;

                let member_exprs = self.builder_gen.regular_members().map(|other_member| {
                    if other_member.ident == self.member.ident {
                        return member_init.clone();
                    }
                    let index = &other_member.index;
                    quote!(self.__private_members.#index)
                });

                quote! {
                    #builder_ident {
                        __private_phantom: ::core::marker::PhantomData,
                        #maybe_receiver_field
                        __private_members: (#( #member_exprs, )*)
                    }
                }
            }
        };

        quote! {
            #( #docs )*
            #[allow(clippy::impl_trait_in_params)]
            #[inline(always)]
            #vis fn #method_name(self, #fn_params) -> #return_type {
                #body
            }
        }
    }

    fn generate_docs_for_setter(&self) -> Vec<syn::Attribute> {
        let member_ident = &self.setter_method_core_name();
        let start_fn_ident = &self.builder_gen.start_func.ident;

        let more = |start_fn_path: &std::fmt::Arguments<'_>| {
            format!(" See {start_fn_path} for more info.")
        };

        let suffix = self
            .builder_gen
            .assoc_method_ctx
            .as_ref()
            .map(|assoc_ctx| {
                let ty = assoc_ctx.self_ty.peel();
                let syn::Type::Path(ty_path) = ty else {
                    // The type is quite complex. It's hard to generate a workable
                    // intra-doc link for it. So in order to avoid the broken
                    // intra-doc links lint we'll just skip adding more info.
                    return "".to_owned();
                };

                let prefix = darling::util::path_to_string(&ty_path.path);
                more(&format_args!("[`{prefix}::{start_fn_ident}()`]"))
            })
            .unwrap_or_else(|| more(&format_args!("[`{start_fn_ident}()`]")));

        let docs = format!("Sets the value of `{member_ident}`.{suffix}");

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
