use super::params::MemberParams;
use super::{params, MemberOrigin};
use crate::builder::builder_gen::builder_params::OnParams;
use crate::builder::builder_gen::member::params::SettersFnParams;
use crate::parsing::SpannedKey;
use crate::util::prelude::*;

/// Regular member for which the builder should have setter methods
#[derive(Debug)]
pub(crate) struct NamedMember {
    /// Specifies what syntax the member comes from.
    pub(crate) origin: MemberOrigin,

    /// Index of the member relative to other regular members. The index is 0-based.
    pub(crate) index: syn::Index,

    /// Original name of the member is used as the name of the builder field and
    /// in its setter methods. Struct field/fn arg names conventionally use `snake_case`
    /// in Rust, but this isn't enforced, so this member isn't guaranteed to be in
    /// snake case, but 99% of the time it will be.
    pub(crate) orig_ident: syn::Ident,

    /// Normalized version of `orig_ident`. Here we stripped the leading `_` from the
    /// member name.
    pub(crate) norm_ident: syn::Ident,

    /// `PascalCase` version of the `norm_ident`.
    pub(crate) norm_ident_pascal: syn::Ident,

    /// Doc comments for the setter methods are copied from the doc comments placed
    /// on top of the original member
    pub(crate) docs: Vec<syn::Attribute>,

    /// Normalized type of the member that the builder should have setters for.
    pub(crate) norm_ty: Box<syn::Type>,

    /// Original type of the member (not normalized)
    pub(crate) orig_ty: Box<syn::Type>,

    /// Parameters configured by the user explicitly via attributes
    pub(crate) params: MemberParams,
}

impl NamedMember {
    pub(super) fn validate(&self) -> Result {
        crate::parsing::reject_self_mentions_in_docs("builder struct's impl block", &self.docs)?;

        if let Some(default) = &self.params.default {
            if !self.params.transparent.is_present() && self.norm_ty.is_option() {
                bail!(
                    &default.key,
                    "`Option<_>` already implies a default of `None`, \
                    so explicit #[builder(default)] is redundant",
                );
            }
        }

        self.validate_setters_params()?;

        if self.params.transparent.is_present() && !self.norm_ty.is_option() {
            bail!(
                &self.params.transparent.span(),
                "`#[builder(transparent)]` can only be applied to members of \
                type `Option<T>` to disable their special handling",
            );
        }

        Ok(())
    }

    fn validate_setters_params(&self) -> Result {
        let setters = match &self.params.setters {
            Some(setters) => setters,
            None => return Ok(()),
        };

        if self.is_required() {
            let SettersFnParams { some_fn, option_fn } = &setters.fns;

            let unexpected_setter = [option_fn, some_fn].into_iter().find_map(Option::as_ref);

            let setter = match unexpected_setter {
                Some(setter) => setter,
                None => return Ok(()),
            };

            bail!(
                &setter.key,
                "`{}` setter function applies only to members with `#[builder(default)]` \
                 or members of `Option<T>` type (if #[builder(transparent)] is not set)",
                setter.key
            );
        }

        if let SettersFnParams {
            some_fn: Some(some_fn),
            option_fn: Some(option_fn),
        } = &setters.fns
        {
            Self::validate_unused_config(&setters.name, &[&some_fn.name, &option_fn.name])?;
            Self::validate_unused_config(&setters.vis, &[&some_fn.vis, &option_fn.vis])?;
            Self::validate_unused_config(&setters.docs, &[&some_fn.docs, &option_fn.docs])?;
        }

        Ok(())
    }

    fn validate_unused_config<T>(
        config: &Option<SpannedKey<T>>,
        overrides: &[&Option<SpannedKey<T>>],
    ) -> Result {
        let config = match config {
            Some(config) => config,
            None => return Ok(()),
        };

        let overrides = overrides.iter().copied().map(Option::as_ref);

        if !overrides.clone().all(|over| over.is_some()) {
            return Ok(());
        }

        let setters = overrides
            .flatten()
            .map(|over| format!("`{}`", over.key))
            .join(", ");

        bail!(
            &config.key,
            "this `{name}` configuration is unused because all of the \
             {setters} setters contain a `{name}` override",
            name = config.key,
        );
    }

    /// Returns the public identifier of the member that should be used in the
    /// generated builder API.
    pub(crate) fn public_ident(&self) -> &syn::Ident {
        self.params.name.as_ref().unwrap_or(&self.norm_ident)
    }

    /// Returns `false` if the member has a default value. It means this member
    /// is required to be set before the building can be finished.
    pub(crate) fn is_required(&self) -> bool {
        self.params.default.is_none()
            && (self.params.transparent.is_present() || !self.norm_ty.is_option())
    }

    /// A stateful member is the one that has a corresponding associated type in
    /// the builder's type state trait. This is used to track the fact that the
    /// member was set or not. This is necessary to make sure all members without
    /// default values are set before the building can be finished.
    pub(crate) fn is_stateful(&self) -> bool {
        self.is_required() || !self.params.overwritable.is_present()
    }

    /// Returns the normalized type of the member stripping the `Option<_>`
    /// wrapper if it's present unless `#[builder(transparent)]` is set.
    pub(crate) fn underlying_norm_ty(&self) -> &syn::Type {
        self.underlying_ty(&self.norm_ty)
    }

    /// Returns the original type of the member stripping the `Option<_>`
    /// wrapper if it's present unless `#[builder(transparent)]` is set.
    pub(crate) fn underlying_orig_ty(&self) -> &syn::Type {
        self.underlying_ty(&self.orig_ty)
    }

    fn underlying_ty<'m>(&'m self, ty: &'m syn::Type) -> &'m syn::Type {
        if self.params.transparent.is_present() || self.params.default.is_some() {
            ty
        } else {
            ty.option_type_param().unwrap_or(ty)
        }
    }

    pub(crate) fn merge_on_params(&mut self, on_params: &[OnParams]) -> Result {
        self.merge_param_into(on_params)?;

        self.params.overwritable = params::EvalBlanketFlagParam {
            on_params,
            param_name: params::BlanketParamName::Overwritable,
            member_params: &self.params,
            scrutinee: &self.norm_ty,
            origin: self.origin,
        }
        .eval()?;

        Ok(())
    }
}
