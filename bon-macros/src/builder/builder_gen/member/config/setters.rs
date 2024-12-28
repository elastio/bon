use crate::parsing::{reject_syntax, SpannedKey};
use crate::util::prelude::*;
use darling::util::SpannedValue;
use darling::FromMeta;
use syn::spanned::Spanned;

const DOCS_CONTEXT: &str = "builder struct's impl block";

fn parse_docs(meta: &syn::Meta) -> Result<SpannedKey<Vec<syn::Attribute>>> {
    crate::parsing::parse_docs_without_self_mentions(DOCS_CONTEXT, meta)
}

#[derive(Debug, FromMeta)]
pub(crate) struct SettersConfig {
    name: Option<SpannedKey<syn::Ident>>,

    #[darling(default, map = Some, with = parse_ident_or_str_lit)]
    prefix: Option<SpannedKey<SpannedValue<String>>>,

    vis: Option<SpannedKey<syn::Visibility>>,

    #[darling(rename = "doc", default, map = Some, with = parse_docs)]
    docs: Option<SpannedKey<Vec<syn::Attribute>>>,

    #[darling(flatten)]
    fns: SettersFnsConfig,
}

#[derive(Debug, FromMeta)]
pub(crate) struct SettersFnsConfig {
    /// Config for the setter that accepts the value of type T for a member of
    /// type `Option<T>` or with `#[builder(default)]`.
    ///
    /// By default, it's named `{member}` without any prefix or suffix.
    pub(crate) some_fn: Option<SpannedKey<SetterFnSigConfig>>,

    /// The setter that accepts the value of type `Option<T>` for a member of
    /// type `Option<T>` or with `#[builder(default)]`.
    ///
    /// By default, it's named `maybe_{member}`.
    pub(crate) option_fn: Option<SpannedKey<SetterFnSigConfig>>,
}

#[derive(Debug, Default)]
pub(crate) struct SetterFnSigConfig {
    pub(crate) name: Option<SpannedKey<syn::Ident>>,
    pub(crate) prefix: Option<SpannedKey<SpannedValue<String>>>,
    pub(crate) vis: Option<SpannedKey<syn::Visibility>>,
    pub(crate) docs: Option<SpannedKey<Vec<syn::Attribute>>>,
}

impl FromMeta for SetterFnSigConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::NameValue(meta) = meta {
            let val = &meta.value;
            let name = syn::parse2(val.to_token_stream())?;

            return Ok(SetterFnSigConfig {
                name: Some(SpannedKey::new(&meta.path, name)?),
                prefix: None,
                vis: None,
                docs: None,
            });
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: Option<SpannedKey<syn::Ident>>,

            #[darling(default, map = Some, with = parse_ident_or_str_lit)]
            prefix: Option<SpannedKey<SpannedValue<String>>>,

            vis: Option<SpannedKey<syn::Visibility>>,

            #[darling(rename = "doc", default, map = Some, with = parse_docs)]
            docs: Option<SpannedKey<Vec<syn::Attribute>>>,
        }

        let Full {
            name,
            prefix,
            vis,
            docs,
        } = crate::parsing::parse_classic_non_empty(meta)?;

        let config = SetterFnSigConfig {
            name,
            prefix,
            vis,
            docs,
        };

        Ok(config)
    }
}

fn parse_ident_or_str_lit(meta: &syn::Meta) -> Result<SpannedKey<SpannedValue<String>>> {
    let expr = darling::util::parse_expr::preserve_str_literal(meta)?;
    let value = match &expr {
        syn::Expr::Lit(syn::ExprLit {
            attrs,
            lit: syn::Lit::Str(str),
            ..
        }) => {
            reject_syntax("attribute", &attrs.first())?;
            str.value()
        }

        syn::Expr::Path(syn::ExprPath {
            attrs, qself, path, ..
        }) => {
            reject_syntax("attribute", &attrs.first())?;
            reject_syntax("<T as Trait> syntax", qself)?;
            path.get_ident()
                .ok_or_else(|| err!(&path, "expected an identifier"))?
                .to_string()
        }

        _ => bail!(&expr, "expected an indetifier or a string literal",),
    };

    let value = SpannedValue::new(value, expr.span());

    SpannedKey::new(meta.path(), value)
}
