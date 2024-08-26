use crate::builder::params::ConditionalParams;
use crate::util::prelude::*;
use darling::util::SpannedValue;
use darling::FromAttributes;
use quote::quote;
use std::fmt;
use syn::spanned::Spanned;

#[derive(Debug, Clone)]
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
    fn parent_construct(&self) -> &'static str {
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

    /// Original name of the member is used as the name of the builder field and
    /// in its setter methods. Struct field/fn arg names conventionally use snake_case
    /// in Rust, but this isn't enforced, so this member isn't guaranteed to be in
    /// snake case, but 99% of the time it will be.
    pub(crate) ident: syn::Ident,

    /// Doc comments for the setter methods are copied from the doc comments placed
    /// on top of the original member
    pub(crate) docs: Vec<syn::Attribute>,

    /// Normalized type of the member that the builder should have setters for.
    pub(crate) norm_ty: Box<syn::Type>,

    /// Original type of the member (not normalized)
    pub(crate) orig_ty: Box<syn::Type>,

    /// The name of the associated type in the builder state trait that corresponds
    /// to this member.
    pub(crate) state_assoc_type_ident: syn::Ident,

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
    fn validate(&self, origin: &MemberOrigin) -> Result {
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

impl Member {
    pub(crate) fn new(
        origin: MemberOrigin,
        attrs: &[syn::Attribute],
        ident: syn::Ident,
        norm_ty: Box<syn::Type>,
        orig_ty: Box<syn::Type>,
    ) -> Result<Self> {
        let docs = attrs.iter().filter(|attr| attr.is_doc()).cloned().collect();

        let params = MemberParams::from_attributes(attrs)?;
        params.validate(&origin)?;

        if let Some(value) = params.skip {
            return Ok(Self::Skipped(SkippedMember {
                ident,
                norm_ty,
                value,
            }));
        }

        let me = RegularMember {
            origin,
            state_assoc_type_ident: ident.snake_to_pascal_case(),
            ident,
            norm_ty,
            orig_ty,
            params,
            docs,
        };

        me.validate()?;

        Ok(Self::Regular(me))
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
            Self::Regular(me) => &me.ident,
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

    pub(crate) fn init_expr(&self) -> TokenStream2 {
        self.as_optional_norm_ty()
            .map(|_| quote!(::bon::private::Optional(::core::marker::PhantomData)))
            .unwrap_or_else(|| quote!(::bon::private::Required(::core::marker::PhantomData)))
    }

    pub(crate) fn unset_state_type(&self) -> TokenStream2 {
        let ty = &self.norm_ty;

        if let Some(inner_type) = self.as_optional_norm_ty() {
            quote!(::bon::private::Optional<#inner_type>)
        } else {
            quote!(::bon::private::Required<#ty>)
        }
    }

    pub(crate) fn set_state_type_param(&self) -> TokenStream2 {
        let ty = &self.norm_ty;

        self.as_optional_norm_ty()
            .map(|ty| quote!(Option<#ty>))
            .unwrap_or_else(|| quote!(#ty))
    }

    pub(crate) fn set_state_type(&self) -> TokenStream2 {
        let ty = self.set_state_type_param();

        quote!(::bon::private::Set<#ty>)
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
}
