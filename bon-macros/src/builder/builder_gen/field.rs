use darling::util::{Flag, SpannedValue};
use darling::FromAttributes;
use prox::prelude::*;
use quote::quote;
use syn::spanned::Spanned;

#[derive(Debug)]
pub(crate) struct Field {
    /// Original name of the field is used as the name of the builder field and
    /// in its setter methods. Field names conventionally use snake_case in Rust,
    /// but this isn't enforced, so this field isn't guaranteed to be in snake case,
    /// but 99% of the time it will be.
    pub(crate) ident: syn::Ident,

    /// Doc comments for the setter methods are copied from the doc comments placed
    /// on top of individual arguments in the original function. Yes, doc comments
    /// are not valid on function arguments in regular Rust, but they are valid if
    /// a proc macro like this one pre-processes them and removes them from the
    /// expanded code.
    pub(crate) docs: Vec<syn::Attribute>,

    /// Type of the function argument that corresponds to this field. This is the
    /// resulting type that the builder should generate setters for.
    pub(crate) ty: Box<syn::Type>,

    /// The name of the associated type in the builder state trait that corresponds
    /// to this field.
    pub(crate) state_assoc_type_ident: syn::Ident,

    /// Parameters configured by the user explicitly via attributes
    pub(crate) params: FieldParams,
}

#[derive(Debug, darling::FromAttributes)]
#[darling(attributes(builder))]
pub(crate) struct FieldParams {
    /// Overrides the decision to use `Into` for the setter method.
    pub(crate) into: Option<bool>,

    #[darling(with = "parse_optional_expression", map = "Some")]
    pub(crate) default: Option<SpannedValue<Option<syn::Expr>>>,

    /// Makes the field required no matter what default treatment for such field
    /// is applied.
    pub(crate) required: Option<Flag>,
}

fn parse_optional_expression(meta: &syn::Meta) -> Result<SpannedValue<Option<syn::Expr>>> {
    match meta {
        syn::Meta::Path(_) => Ok(SpannedValue::new(None, meta.span())),
        syn::Meta::List(_) => Err(Error::unsupported_format("list").with_span(meta)),
        syn::Meta::NameValue(nv) => Ok(SpannedValue::new(Some(nv.value.clone()), nv.span())),
    }
}

impl Field {
    pub(crate) fn new(
        attrs: &[syn::Attribute],
        ident: syn::Ident,
        ty: Box<syn::Type>,
    ) -> Result<Self> {
        let docs = attrs.iter().filter(|attr| attr.is_doc()).cloned().collect();

        let params = FieldParams::from_attributes(attrs)?;

        let me = Self {
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
            let ty = if self.ty.is_option() {
                Some("Option")
            } else if self.ty.is_bool() {
                Some("bool")
            } else {
                None
            };

            if let Some(ty) = ty {
                prox::bail!(
                    &default.span(),
                    "type `{ty}` already implies #[builder(default)] \
                    so explicit #[builder(default)] is redundant",
                );
            }
        }

        if let Some(required) = &self.params.required {
            if self.ty.is_option() {
                prox::bail!(
                    &required.span(),
                    "`Option` and #[builder(required)] attributes are mutually exclusive"
                );
            }

            if self.params.default.is_some() {
                prox::bail!(
                    &required.span(),
                    "The #[builder(required)] and #[builder(default)] attributes \
                   are mutually exclusive"
                );
            }
        }

        Ok(())
    }

    pub(crate) fn as_optional(&self) -> Option<&syn::Type> {
        // User override takes the wheel entirely
        if self.params.required.is_some() {
            return None;
        }

        self.ty
            .option_type_param()
            .or_else(|| (self.ty.is_bool() || self.params.default.is_some()).then_some(&self.ty))
    }

    pub(crate) fn unset_state_type(&self) -> TokenStream2 {
        let ty = &self.ty;

        if let Some(inner_type) = self.as_optional() {
            quote!(bon::private::Optional<#inner_type>)
        } else {
            quote!(bon::private::Required<#ty>)
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

        quote!(bon::private::Set<#ty>)
    }
}
