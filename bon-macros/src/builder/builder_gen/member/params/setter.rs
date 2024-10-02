use crate::parsing::{ItemParams, ItemParamsParsing};
use crate::util::prelude::*;
use darling::FromMeta;

const DOCS_CONTEXT: &str = "builder struct's impl block";

fn parse_setter_fn(meta: &syn::Meta) -> Result<ItemParams> {
    ItemParamsParsing {
        meta,
        reject_self_mentions: Some(DOCS_CONTEXT),
    }
    .parse()
}

fn parse_docs(meta: &syn::Meta) -> Result<Vec<syn::Attribute>> {
    crate::parsing::parse_docs_without_self_mentions(DOCS_CONTEXT, meta)
}

#[derive(Debug, FromMeta)]
pub(crate) struct SettersParams {
    pub(crate) name: Option<syn::Ident>,
    pub(crate) vis: Option<syn::Visibility>,

    #[darling(default, with = parse_docs, map = Some)]
    pub(crate) docs: Option<Vec<syn::Attribute>>,

    /// Config for the setter that accepts the value of type T for a member of
    /// type `Option<T>` or with `#[builder(default)]`.
    ///
    /// By default, it's named `{member}` without any prefix or suffix.
    #[darling(default, with = parse_setter_fn)]
    pub(crate) some_fn: ItemParams,

    /// The setter that accepts the value of type `Option<T>` for a member of
    /// type `Option<T>` or with `#[builder(default)]`.
    ///
    /// By default, it's named `maybe_{member}`.
    #[darling(default, with = parse_setter_fn)]
    pub(crate) option_fn: ItemParams,
}
