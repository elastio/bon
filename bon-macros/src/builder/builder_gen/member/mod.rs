mod into_conversion;
mod params;

use crate::util::prelude::*;
use darling::util::SpannedValue;
use darling::FromAttributes;
use params::{MemberInputSource, MemberParams};
use quote::quote;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub(crate) enum MemberOrigin {
    FnArg,
    StructField,
}

impl fmt::Display for MemberOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FnArg => write!(f, "function argument"),
            Self::StructField => write!(f, "struct field"),
        }
    }
}

impl MemberOrigin {
    fn parent_construct(self) -> &'static str {
        match self {
            Self::FnArg => "function",
            Self::StructField => "struct",
        }
    }
}

#[derive(Debug)]
pub(crate) enum Member {
    Named(NamedMember),

    /// Member that was marked with `#[builder(pos = start_fn)]`
    StartFnArg(StartFnArgMember),

    /// Member that was marked with `#[builder(pos = finish_fn)]`
    FinishFnArg(PositionalFnArgMember),

    Skipped(SkippedMember),
}

/// Regular member for which the builder should have setter methods
#[derive(Debug, Clone)]
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

    /// The name of the type variable that can be used as the type of this
    /// member in contexts where it should be generic.
    pub(crate) generic_var_ident: syn::Ident,

    /// Parameters configured by the user explicitly via attributes
    pub(crate) params: MemberParams,
}

/// Member that was marked with `#[builder(pos = start_fn)]`
#[derive(Debug)]
pub(crate) struct StartFnArgMember {
    pub(crate) base: PositionalFnArgMember,

    /// Index of the member relative to other positional members. The index is 0-based.
    pub(crate) index: syn::Index,
}

#[derive(Debug)]
pub(crate) struct PositionalFnArgMember {
    /// Specifies what syntax the member comes from.
    pub(crate) origin: MemberOrigin,

    /// Original identifier of the member
    pub(crate) ident: syn::Ident,

    /// Normalized type of the member
    pub(crate) norm_ty: Box<syn::Type>,

    /// Original type of the member (not normalized)
    pub(crate) orig_ty: Box<syn::Type>,

    /// Parameters configured by the user explicitly via attributes
    pub(crate) params: MemberParams,
}

/// Member that was skipped by the user with `#[builder(skip)]`
#[derive(Debug)]
pub(crate) struct SkippedMember {
    pub(crate) ident: syn::Ident,

    pub(crate) norm_ty: Box<syn::Type>,

    /// Value to assign to the member
    pub(crate) value: SpannedValue<Option<syn::Expr>>,
}

impl NamedMember {
    fn validate(&self) -> Result {
        super::reject_self_references_in_docs(&self.docs)?;

        if let Some(default) = &self.params.default {
            if self.norm_ty.is_option() {
                bail!(
                    &default.span(),
                    "`Option<_>` already implies a default of `None`, \
                    so explicit #[builder(default)] is redundant",
                );
            }
        }

        Ok(())
    }

    fn as_optional_with_ty<'a>(&'a self, ty: &'a syn::Type) -> Option<&'a syn::Type> {
        ty.option_type_param()
            .or_else(|| (self.params.default.is_some()).then(|| ty))
    }

    pub(crate) fn as_optional_norm_ty(&self) -> Option<&syn::Type> {
        Self::as_optional_with_ty(self, &self.norm_ty)
    }

    pub(crate) fn is_optional(&self) -> bool {
        self.as_optional_norm_ty().is_some()
    }

    /// The type parameter for the `Set<T>` type that corresponds to this member
    pub(crate) fn set_state_type_param(&self) -> TokenStream2 {
        let ty = &self.norm_ty;
        let ty = self
            .as_optional_norm_ty()
            .map(|ty| quote!(Option<#ty>))
            .unwrap_or_else(|| quote!(#ty));

        quote!(#ty)
    }

    pub(crate) fn param_default(&self) -> Option<Option<&syn::Expr>> {
        self.params
            .default
            .as_ref()
            .map(|default| default.as_ref().as_ref())
    }
}

pub(crate) struct RawMember<'a> {
    pub(crate) attrs: &'a [syn::Attribute],
    pub(crate) ident: syn::Ident,
    pub(crate) norm_ty: Box<syn::Type>,
    pub(crate) orig_ty: Box<syn::Type>,
}

impl Member {
    // False-positive lint. We can't elide the lifetime in `RawMember` because
    // anonymous lifetimes in impl traits are unstable, and we shouldn't omit
    // the lifetime parameter because we want to be explicit about its existence
    // (there is an other lint that checks for this).
    #[allow(single_use_lifetimes)]
    pub(crate) fn from_raw<'a>(
        origin: MemberOrigin,
        members: impl IntoIterator<Item = RawMember<'a>>,
    ) -> Result<Vec<Self>> {
        let mut named_count = 0;
        let mut start_fn_arg_count = 0;

        members
            .into_iter()
            .map(|member| {
                let RawMember {
                    attrs,
                    ident: orig_ident,
                    norm_ty,
                    orig_ty,
                } = member;

                let params = MemberParams::from_attributes(attrs)?;
                params.validate(origin)?;

                if let Some(value) = params.skip {
                    return Ok(Self::Skipped(SkippedMember {
                        ident: orig_ident,
                        norm_ty,
                        value,
                    }));
                }

                if let Some(pos) = params.source {
                    let base = PositionalFnArgMember {
                        origin,
                        ident: orig_ident,
                        norm_ty,
                        orig_ty,
                        params,
                    };
                    match pos {
                        MemberInputSource::StartFn(_) => {
                            let index = start_fn_arg_count.into();
                            start_fn_arg_count += 1;
                            return Ok(Self::StartFnArg(StartFnArgMember { base, index }));
                        }
                        MemberInputSource::FinishFn(_) => {
                            return Ok(Self::FinishFnArg(base));
                        }
                    }
                }

                // XXX: docs are collected only for named members. There is obvious
                // place where to put the docs for positional and skipped members.
                //
                // Even if there are some docs on them and the function syntax is used
                // then these docs will just be removed from the output function.
                // It's probably fine since the doc comments are there in the code
                // itself which is also useful for people reading the source code.
                let docs = attrs.iter().filter(|attr| attr.is_doc()).cloned().collect();

                let orig_ident_str = orig_ident.to_string();
                let norm_ident = orig_ident_str
                    // Remove the leading underscore from the member name since it's used
                    // to denote unused symbols in Rust. That doesn't mean the builder
                    // API should expose that knowledge to the caller.
                    .strip_prefix('_')
                    .unwrap_or(&orig_ident_str);

                // Preserve the original identifier span to make IDE go to definition correctly
                // and make error messages point to the correct place.
                let norm_ident = syn::Ident::new_maybe_raw(norm_ident, orig_ident.span());
                let norm_ident_pascal = norm_ident.snake_to_pascal_case();

                let me = NamedMember {
                    index: named_count.into(),
                    origin,
                    generic_var_ident: quote::format_ident!("__{}", norm_ident_pascal),
                    norm_ident_pascal,
                    orig_ident,
                    norm_ident,
                    norm_ty,
                    orig_ty,
                    params,
                    docs,
                };

                named_count += 1;

                me.validate()?;

                Ok(Self::Named(me))
            })
            .collect()
    }
}

impl Member {
    pub(crate) fn norm_ty(&self) -> &syn::Type {
        match self {
            Self::Named(me) => &me.norm_ty,
            Self::StartFnArg(me) => &me.base.norm_ty,
            Self::FinishFnArg(me) => &me.norm_ty,
            Self::Skipped(me) => &me.norm_ty,
        }
    }

    pub(crate) fn orig_ident(&self) -> &syn::Ident {
        match self {
            Self::Named(me) => &me.orig_ident,
            Self::StartFnArg(me) => &me.base.ident,
            Self::FinishFnArg(me) => &me.ident,
            Self::Skipped(me) => &me.ident,
        }
    }

    pub(crate) fn as_named(&self) -> Option<&NamedMember> {
        match self {
            Self::Named(me) => Some(me),
            _ => None,
        }
    }

    pub(crate) fn as_start_fn_arg(&self) -> Option<&StartFnArgMember> {
        match self {
            Self::StartFnArg(me) => Some(me),
            _ => None,
        }
    }

    pub(crate) fn as_finish_fn_arg(&self) -> Option<&PositionalFnArgMember> {
        match self {
            Self::FinishFnArg(me) => Some(me),
            _ => None,
        }
    }
}