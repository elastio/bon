use super::params::{BlanketParamName, EvalBlanketFlagParam};
use super::{NamedMember, PositionalFnArgMember};
use crate::builder::builder_gen::builder_params::OnParams;
use crate::util::prelude::*;

impl NamedMember {
    pub(super) fn merge_param_into(&mut self, on_params: &[OnParams]) -> Result {
        // `with` is mutually exclusive with `into`. So there is nothing to merge here
        // if `with` is present.
        if self.params.with.is_some() {
            return Ok(());
        }

        // For optional named members the target of the `Into` conversion is the type
        // inside of the `Option<T>`, not the `Option<T>` itself because we generate
        // a setter that accepts `T` itself. It also makes this logic stable regardless
        // if `Option<T>` is used or the member of type `T` has `#[builder(default)]` on it.
        let scrutinee = self.underlying_orig_ty();

        self.params.into = EvalBlanketFlagParam {
            on_params,
            param_name: BlanketParamName::Into,
            member_params: &self.params,
            scrutinee,
            origin: self.origin,
        }
        .eval()?;

        Ok(())
    }
}

impl PositionalFnArgMember {
    pub(crate) fn merge_param_into(&mut self, on_params: &[OnParams]) -> Result {
        // Positional members are never optional. Users must always specify them, so there
        // is no need for us to look into the `Option<T>` generic parameter, because the
        // `Option<T>` itself is the target of the into conversion, not the `T` inside it.
        let scrutinee = self.orig_ty.as_ref();

        self.params.into = EvalBlanketFlagParam {
            on_params,
            param_name: BlanketParamName::Into,
            member_params: &self.params,
            scrutinee,
            origin: self.origin,
        }
        .eval()?;

        Ok(())
    }

    pub(crate) fn fn_input_param(&self) -> TokenStream {
        let has_into = self.params.into.is_present();
        let norm_ty = &self.norm_ty;
        let ident = &self.ident;

        if has_into {
            quote! { #ident: impl Into<#norm_ty> }
        } else {
            quote! { #ident: #norm_ty }
        }
    }

    pub(crate) fn maybe_into_ident_expr(&self) -> TokenStream {
        let has_into = self.params.into.is_present();
        let ident = &self.ident;

        if has_into {
            quote! {
                ::core::convert::Into::into(#ident)
            }
        } else {
            ident.to_token_stream()
        }
    }
}
