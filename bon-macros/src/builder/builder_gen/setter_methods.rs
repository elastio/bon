use super::{BuilderGenCtx, RegularMember};
use crate::builder::builder_gen::AssocMethodCtx;
use crate::util::prelude::*;
use quote::{quote, ToTokens};

impl BuilderGenCtx {
    pub(crate) fn setter_methods_impls_for_member(
        &self,
        member: &RegularMember,
    ) -> Result<TokenStream2> {
        let output_members_states = self.regular_members().map(|other_member| {
            if other_member.ident == member.ident {
                return member.set_state_type().to_token_stream();
            }

            let state_assoc_type_ident = &other_member.state_assoc_type_ident;
            quote!(__State::#state_assoc_type_ident)
        });

        let state_assoc_type_ident = &member.state_assoc_type_ident;
        let builder_ident = &self.builder_ident;
        let builder_state_trait_ident = &self.builder_state_trait_ident;
        let generics_decl = &self.generics.params;
        let generic_args: Vec<_> = self.generic_args().collect();
        let where_clause = &self.generics.where_clause;
        let unset_state_type = member.unset_state_type();
        let output_builder_alias_ident = quote::format_ident!(
            "__{}Set{}",
            builder_ident.raw_name(),
            state_assoc_type_ident.raw_name()
        );

        // A case where there is just one member is special, because the type alias would
        // receive a generic `__State` parameter that it wouldn't use, so we create it
        // only if there are 2 or more members.
        let (output_builder_alias_state_var_decl, output_builder_alias_state_arg) =
            (self.regular_members().count() > 1)
                .then(|| (quote!(__State: #builder_state_trait_ident), quote!(__State)))
                .unzip();

        let setter_methods = MemberSettersCtx::new(
            self,
            member,
            quote! {
                #output_builder_alias_ident<
                    #(#generic_args,)*
                    #output_builder_alias_state_arg
                >
            },
        )
        .setter_methods()?;

        let vis = &self.vis;

        Ok(quote! {
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
            // This is `doc(hidden)` with the same visibility as the setter to reduce the noise in
            // the docs generated by `rustdoc`. Rustdoc auto-inlines type aliases if they aren't exposed
            // as part of the public API of the crate. This is a workaround to prevent that.
            #[doc(hidden)]
            #vis type #output_builder_alias_ident<
                #(#generics_decl,)*
                #output_builder_alias_state_var_decl
            >
            // The where clause in this position will be deprecated. The preferred
            // position will be at the end of the entire type alias syntax construct.
            // See details at https://github.com/rust-lang/rust/issues/112792.
            //
            // However, at the time of this writing the only way to put the where
            // clause on a type alias is here.
            #where_clause
            = #builder_ident<
                #(#generic_args,)*
                ( #(#output_members_states,)* )
            >;

            impl<
                #(#generics_decl,)*
                __State: #builder_state_trait_ident<
                    #state_assoc_type_ident = #unset_state_type
                >
            >
            #builder_ident<
                #(#generic_args,)*
                __State
            >
            #where_clause
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

    fn setter_method_name(&self) -> syn::Ident {
        self.member
            .params
            .name
            .clone()
            .unwrap_or_else(|| self.norm_member_ident.clone())
    }

    fn setter_methods(&self) -> Result<TokenStream2> {
        let member_type = self.member.ty.as_ref();

        if let Some(inner_type) = self.member.as_optional() {
            return self.setters_for_optional_member(inner_type);
        }

        let qualified_for_into = self
            .builder_gen
            .member_qualifies_for_into(self.member, &self.member.ty)?;

        let (fn_param_type, maybe_into_call) = if qualified_for_into {
            (quote!(impl Into<#member_type>), quote!(.into()))
        } else {
            (quote!(#member_type), quote!())
        };

        Ok(self.setter_method(MemberSetterMethod {
            method_name: self.setter_method_name(),
            fn_params: quote!(value: #fn_param_type),
            member_init: quote!(::bon::private::Set(value #maybe_into_call)),
            overwrite_docs: None,
        }))
    }

    fn setters_for_optional_member(&self, inner_type: &syn::Type) -> Result<TokenStream2> {
        let qualified_for_into = self
            .builder_gen
            .member_qualifies_for_into(self.member, inner_type)?;

        let (inner_type, maybe_conv_call, maybe_map_conv_call) = if qualified_for_into {
            (
                quote!(impl Into<#inner_type>),
                quote!(.into()),
                quote!(.map(Into::into)),
            )
        } else {
            (quote!(#inner_type), quote!(), quote!())
        };

        let setter_method_name = self.setter_method_name();

        // Preserve the original identifier span to make IDE go to definition correctly
        let option_method_name = syn::Ident::new(
            &format!("maybe_{}", setter_method_name.raw_name()),
            setter_method_name.span(),
        );

        let methods = [
            MemberSetterMethod {
                method_name: option_method_name,
                fn_params: quote!(value: Option<#inner_type>),
                member_init: quote!(::bon::private::Set(value #maybe_map_conv_call)),
                overwrite_docs: Some(format!(
                    "Same as [`Self::{setter_method_name}`], but accepts \
                    an `Option` as input. See that method's documentation for \
                    more details.",
                )),
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
                member_init: quote!(::bon::private::Set(Some(value #maybe_conv_call))),
                overwrite_docs: None,
            },
        ];

        let setters = methods
            .into_iter()
            .map(|method| self.setter_method(method))
            .collect();

        Ok(setters)
    }

    fn setter_method(&self, method: MemberSetterMethod) -> TokenStream2 {
        let return_type = &self.return_type;
        let MemberSetterMethod {
            method_name,
            fn_params,
            member_init,
            overwrite_docs,
        } = method;

        let docs = match overwrite_docs {
            Some(docs) => vec![syn::parse_quote!(#[doc = #docs])],
            None => self.member.docs.clone(),
        };

        let vis = &self.builder_gen.vis;

        let builder_ident = &self.builder_gen.builder_ident;
        let builder_private_impl_ident = &self.builder_gen.builder_private_impl_ident;
        let member_idents = self
            .builder_gen
            .regular_members()
            .map(|member| member.ident.clone());

        let maybe_receiver_field = self
            .builder_gen
            .assoc_method_ctx
            .as_ref()
            .and_then(AssocMethodCtx::as_receiver)
            .is_some()
            .then(|| quote!(receiver: self.__private_impl.receiver,));

        let member_exprs = self.builder_gen.regular_members().map(|other_member| {
            if other_member.ident == self.member.ident {
                return member_init.clone();
            }

            let ident = &other_member.ident;
            quote!(self.__private_impl.#ident)
        });

        quote! {
            #( #docs )*
            #[inline(always)]
            #vis fn #method_name(self, #fn_params) -> #return_type {
                #builder_ident {
                    __private_impl: #builder_private_impl_ident {
                        _phantom: ::core::marker::PhantomData,
                        #maybe_receiver_field
                        #( #member_idents: #member_exprs, )*
                    }
                }
            }
        }
    }
}

struct MemberSetterMethod {
    method_name: syn::Ident,
    fn_params: TokenStream2,
    member_init: TokenStream2,
    overwrite_docs: Option<String>,
}
