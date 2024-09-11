use super::{MemberOrigin, MemberParams, NamedMember, PositionalFnArgMember};
use crate::builder::params::ConditionalParams;
use crate::util::prelude::*;
use quote::{quote, ToTokens};

impl NamedMember {
    pub(crate) fn param_into(&self, conditional_params: &[ConditionalParams]) -> Result<bool> {
        // For optional named members the target of the `Into` conversion is the type
        // inside of the `Option<T>`, not the `Option<T>` itself because we generate
        // a setter that accepts `T` itself. It also makes this logic stable regardless
        // if `Option<T>` is used or the member of type `T` has `#[builder(default)]` on it.
        let scrutinee = self
            .as_optional_with_ty(&self.orig_ty)
            .unwrap_or(&self.orig_ty);

        is_into_enabled(self.origin, &self.params, scrutinee, conditional_params)
    }

    pub(crate) fn setter_method_core_name(&self) -> &syn::Ident {
        self.params.name.as_ref().unwrap_or(&self.norm_ident)
    }
}

impl PositionalFnArgMember {
    pub(crate) fn param_into(&self, conditional_params: &[ConditionalParams]) -> Result<bool> {
        // Positional members are never optional. Users must always specify them, so there
        // is no need for us to look into the `Option<T>` generic parameter, because the
        // `Option<T>` itself is the target of the into conversion, not the `T` inside it.
        let scrutinee = self.orig_ty.as_ref();

        is_into_enabled(self.origin, &self.params, scrutinee, conditional_params)
    }

    pub(crate) fn fn_input_param(
        &self,
        conditional_params: &[ConditionalParams],
    ) -> Result<TokenStream2> {
        let has_into = self.param_into(conditional_params)?;
        let norm_ty = &self.norm_ty;
        let ident = &self.ident;

        let input = if has_into {
            quote! { #ident: impl Into<#norm_ty> }
        } else {
            quote! { #ident: #norm_ty }
        };

        Ok(input)
    }

    pub(crate) fn maybe_into_ident_expr(
        &self,
        conditional_params: &[ConditionalParams],
    ) -> Result<TokenStream2> {
        let has_into = self.param_into(conditional_params)?;
        let ident = &self.ident;

        let expr = if has_into {
            quote! {
                ::core::convert::Into::into(#ident)
            }
        } else {
            ident.to_token_stream()
        };

        Ok(expr)
    }
}

fn is_into_enabled(
    origin: MemberOrigin,
    member_params: &MemberParams,
    scrutinee: &syn::Type,
    conditional_params: &[ConditionalParams],
) -> Result<bool> {
    let verdict_from_defaults = conditional_params
        .iter()
        .map(|params| Ok((params, scrutinee.matches(&params.type_pattern)?)))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|(_, matched)| *matched)
        .any(|(params, _)| params.into.is_present());

    let verdict_from_override = member_params.into.is_present();

    if verdict_from_defaults && verdict_from_override {
        bail!(
            &member_params.into.span(),
            "this `#[builder(into)]` attribute is redundant, because `into` \
            is already implied for this member via the `#[builder(on(...))]` \
            at the top of the {}",
            origin.parent_construct(),
        );
    }

    Ok(verdict_from_override || verdict_from_defaults)
}
