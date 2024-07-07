use darling::util::parse_expr::preserve_str_literal;
use darling::FromAttributes;
use prox::prelude::*;
use quote::quote;

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

#[derive(darling::FromAttributes)]
#[darling(attributes(builder))]
pub(crate) struct FieldParams {
    /// Overrides the decision to use `Into` for the setter method.
    pub(crate) into: Option<bool>,

    #[darling(with = "preserve_str_literal", map = "map_optional_expr")]
    pub(crate) default: Option<Option<syn::Expr>>,
}

// Wrapping is intentional and basically the purpose of this function
#[allow(clippy::unnecessary_wraps)]
fn map_optional_expr(meta: syn::Expr) -> Option<Option<syn::Expr>> {
    Some(Some(meta))
}

impl Field {
    pub(crate) fn from_typed_fn_arg(arg: &syn::PatType) -> Result<Self> {
        let syn::Pat::Ident(pat) = arg.pat.as_ref() else {
            // We may allow setting a name for the builder method in parameter
            // attributes and relax this requirement
            prox::bail!(
                &arg.pat,
                "Only simple identifiers in function arguments supported \
                to infer the name of builder methods"
            );
        };

        let docs = arg
            .attrs
            .iter()
            .filter(|attr| attr.is_doc())
            .cloned()
            .collect();

        let params = FieldParams::from_attributes(&arg.attrs)?;

        Ok(Self {
            state_assoc_type_ident: pat.ident.to_pascal_case(),
            ident: pat.ident.clone(),
            ty: arg.ty.clone(),
            params,
            docs,
        })
    }

    fn is_optional(&self) -> bool {
        self.ty.is_option() || self.ty.is_bool() || self.params.default.is_some()
    }

    pub(crate) fn unset_state_type(&self) -> TokenStream2 {
        let ty = &self.ty;

        let state = if self.is_optional() {
            quote!(Optional)
        } else {
            quote!(Required)
        };

        quote!(bon::private::#state<#ty>)
    }

    pub(crate) fn unset_state_init_expr(&self) -> TokenStream2 {
        if !self.is_optional() {
            return quote!(bon::private::Required::default());
        }

        let default_fn = self
            .params
            .default
            .as_ref()
            .map(|default| quote! { || #default })
            .unwrap_or_else(|| quote! { Default::default });

        quote!(bon::private::Optional::new(#default_fn))
    }

    pub(crate) fn set_state_type(&self) -> TokenStream2 {
        let ty = &self.ty;
        quote!(bon::private::Set<#ty>)
    }
}
