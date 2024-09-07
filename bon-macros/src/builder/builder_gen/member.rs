use crate::builder::params::ConditionalParams;
use crate::util::prelude::*;
use darling::util::SpannedValue;
use darling::FromAttributes;
use quote::quote;
use std::fmt;
use syn::spanned::Spanned;

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
    Regular(RegularMember),
    Skipped(SkippedMember),
}

/// Regular member for which the builder should have setter methods
#[derive(Debug, Clone)]
pub(crate) struct RegularMember {
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

/// Member that was skipped by the user with `#[builder(skip)]`
#[derive(Debug)]
pub(crate) struct SkippedMember {
    pub(crate) ident: syn::Ident,

    pub(crate) norm_ty: Box<syn::Type>,

    /// Value to assign to the member
    pub(crate) value: SpannedValue<Option<syn::Expr>>,
}

#[derive(Debug, Clone, darling::FromAttributes)]
#[darling(attributes(builder))]
pub(crate) struct MemberParams {
    /// Enables an `Into` conversion for the setter method.
    pub(crate) into: darling::util::Flag,

    /// Assign a default value to the member it it's not specified.
    ///
    /// An optional expression can be provided to set the value for the member,
    /// otherwise its  [`Default`] trait impl will be used.
    #[darling(with = parse_optional_expression, map = Some)]
    pub(crate) default: Option<SpannedValue<Option<syn::Expr>>>,

    /// Skip generating a setter method for this member.
    ///
    /// An optional expression can be provided to set the value for the member,
    /// otherwise its  [`Default`] trait impl will be used.
    #[darling(with = parse_optional_expression, map = Some)]
    pub(crate) skip: Option<SpannedValue<Option<syn::Expr>>>,

    /// Rename the name exposed in the builder API.
    pub(crate) name: Option<syn::Ident>,
}

impl MemberParams {
    fn validate(&self, origin: MemberOrigin) -> Result {
        if let Self {
            skip: Some(skip),
            into,
            default,
            name,
        } = self
        {
            match origin {
                MemberOrigin::FnArg => {
                    bail!(
                        &skip.span(),
                        "`skip` attribute is not supported on function arguments. \
                        Use a local variable instead.",
                    );
                }
                MemberOrigin::StructField => {}
            }

            let other_attr = [
                default.as_ref().map(|attr| ("default", attr.span())),
                name.as_ref().map(|attr| ("name", attr.span())),
                into.is_present().then(|| ("into", into.span())),
            ]
            .into_iter()
            .flatten()
            .next();

            if let Some((attr_name, span)) = other_attr {
                let default_hint = if default.as_ref().is_some_and(|expr| expr.is_some()) {
                    ". If you wanted to specify a value for the member, then use \
                    the following syntax instead `#[builder(skip = value)]`"
                } else {
                    ""
                };

                bail!(
                    &span,
                    "`skip` attribute can't be specified with other attributes like `{}` \
                    because there will be no setter generated for this member to configure{default_hint}",
                    attr_name,
                );
            }
        }

        Ok(())
    }
}

fn parse_optional_expression(meta: &syn::Meta) -> Result<SpannedValue<Option<syn::Expr>>> {
    match meta {
        syn::Meta::Path(_) => Ok(SpannedValue::new(None, meta.span())),
        syn::Meta::List(_) => Err(Error::unsupported_format("list").with_span(meta)),
        syn::Meta::NameValue(nv) => Ok(SpannedValue::new(Some(nv.value.clone()), nv.span())),
    }
}

impl RegularMember {
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
            .or_else(|| (self.params.default.is_some()).then_some(ty))
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

    pub(crate) fn param_into(&self, conditional_params: &[ConditionalParams]) -> Result<bool> {
        let scrutinee = self
            .as_optional_with_ty(&self.orig_ty)
            .unwrap_or(&self.orig_ty);

        let verdict_from_defaults = conditional_params
            .iter()
            .map(|params| Ok((params, scrutinee.matches(&params.type_pattern)?)))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter(|(_, matched)| *matched)
            .any(|(params, _)| params.into.is_present());

        let verdict_from_override = self.params.into.is_present();

        if verdict_from_defaults && verdict_from_override {
            bail!(
                &self.params.into.span(),
                "this `#[builder(into)]` attribute is redundant, because `into` \
                is already implied for this member via the `#[builder(on(...))]` \
                at the top of the {}",
                self.origin.parent_construct(),
            );
        }

        Ok(verdict_from_override || verdict_from_defaults)
    }

    pub(crate) fn setter_method_core_name(&self) -> &syn::Ident {
        self.params.name.as_ref().unwrap_or(&self.norm_ident)
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
        let mut regular_members_count = 0;

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

                let me = RegularMember {
                    index: regular_members_count.into(),
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

                regular_members_count += 1;

                me.validate()?;

                Ok(Self::Regular(me))
            })
            .collect()
    }
}

impl Member {
    pub(crate) fn norm_ty(&self) -> &syn::Type {
        match self {
            Self::Regular(me) => &me.norm_ty,
            Self::Skipped(me) => &me.norm_ty,
        }
    }

    pub(crate) fn ident(&self) -> &syn::Ident {
        match self {
            Self::Regular(me) => &me.orig_ident,
            Self::Skipped(me) => &me.ident,
        }
    }

    pub(crate) fn as_regular(&self) -> Option<&RegularMember> {
        match self {
            Self::Regular(me) => Some(me),
            Self::Skipped(_) => None,
        }
    }
}
