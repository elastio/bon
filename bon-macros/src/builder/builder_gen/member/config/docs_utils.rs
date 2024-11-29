use crate::util::prelude::Result;

pub(crate) const DOCS_CONTEXT: &str = "builder struct's impl block";

pub(crate) fn parse_docs(meta: &syn::Meta) -> Result<super::SpannedKey<Vec<syn::Attribute>>> {
    crate::parsing::parse_docs_without_self_mentions(DOCS_CONTEXT, meta)
}
