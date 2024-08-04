use crate::util::prelude::*;
use darling::util::SpannedValue;
use darling::{FromAttributes, FromMeta};
use quote::quote;
use std::fmt;
use syn::spanned::Spanned;

#[derive(Debug)]
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

#[derive(Debug)]
pub(crate) struct Member {
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

    /// Type of member that the builder should have setters for.
    pub(crate) ty: Box<syn::Type>,

    /// The name of the associated type in the builder state trait that corresponds
    /// to this member.
    pub(crate) state_assoc_type_ident: syn::Ident,

    /// Parameters configured by the user explicitly via attributes
    pub(crate) params: MemberParams,
}

#[derive(Debug, darling::FromAttributes)]
#[darling(attributes(builder))]
pub(crate) struct MemberParams {
    /// Overrides the decision to use `Into` for the setter method.
    pub(crate) into: Option<SpannedValue<StrictBool>>,

    #[darling(with = "parse_optional_expression", map = "Some")]
    pub(crate) default: Option<SpannedValue<Option<syn::Expr>>>,

    /// Rename the name exposed in the builder API.
    pub(crate) name: Option<syn::Ident>,
}

/// This primitive represents the syntax that accepts only two states:
/// a word e.g. `#[attr(field)]` represents true, and an expression with
/// `false` e.g. `#[attr(field = false)]` represents false. No other syntax
/// is accepted. That's why it's called a "strict" bool.
#[derive(Debug)]
pub(crate) struct StrictBool {
    pub(crate) value: bool,
}

impl FromMeta for StrictBool {
    fn from_word() -> Result<Self> {
        Ok(Self { value: true })
    }

    fn from_bool(value: bool) -> Result<Self> {
        if !value {
            return Ok(Self { value: false });
        }

        // Error span is set by default trait impl in the caller
        Err(Error::custom(format_args!(
            "No need to write `= true`. Just mentioning the attribute is enough \
            to set it to `true`, so remove the `= true` part.",
        )))
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
        ident: Option<syn::Ident>,
        ty: Box<syn::Type>,
    ) -> Result<Self> {
        let docs = attrs.iter().filter(|attr| attr.is_doc()).cloned().collect();

        let params = MemberParams::from_attributes(attrs)?;

        let ident = ident.or_else(|| params.name.clone()).ok_or_else(|| {
            err!(
                &ty,
                "can't infer the name to use for this {origin}; please use a simple \
                `identifier: type` syntax for the {origin}, or add \
                `#[builder(name = explicit_name)]` to specify the name explicitly",
            )
        })?;

        let me = Self {
            origin,
            state_assoc_type_ident: ident.to_pascal_case(),
            ident,
            ty,
            params,
            docs,
        };

        me.validate()?;

        Ok(me)
    }

    fn validate(&self) -> Result {
        super::reject_self_references_in_docs(&self.docs)?;

        if let Some(default) = &self.params.default {
            if self.ty.is_option() {
                bail!(
                    &default.span(),
                    "`Option<_>` already implies a default of `None`, \
                    so explicit #[builder(default)] is redundant",
                );
            }
        }

        Ok(())
    }

    pub(crate) fn as_optional(&self) -> Option<&syn::Type> {
        self.ty
            .option_type_param()
            .or_else(|| (self.params.default.is_some()).then_some(&self.ty))
    }

    pub(crate) fn init_expr(&self) -> TokenStream2 {
        self.as_optional()
            .map(|_| quote!(::bon::private::Optional(::std::marker::PhantomData)))
            .unwrap_or_else(|| quote!(::bon::private::Required(::std::marker::PhantomData)))
    }

    pub(crate) fn unset_state_type(&self) -> TokenStream2 {
        let ty = &self.ty;

        if let Some(inner_type) = self.as_optional() {
            quote!(::bon::private::Optional<#inner_type>)
        } else {
            quote!(::bon::private::Required<#ty>)
        }
    }

    pub(crate) fn set_state_type_param(&self) -> TokenStream2 {
        let ty = &self.ty;

        self.as_optional()
            .map(|ty| quote!(Option<#ty>))
            .unwrap_or_else(|| quote!(#ty))
    }

    pub(crate) fn set_state_type(&self) -> TokenStream2 {
        let ty = self.set_state_type_param();

        quote!(::bon::private::Set<#ty>)
    }
}
