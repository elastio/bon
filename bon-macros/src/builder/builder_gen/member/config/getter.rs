use darling::FromMeta;

use super::{Result, SpannedKey};

#[derive(Debug, Default)]
pub(crate) struct GetterConfig {
    name: Option<SpannedKey<syn::Ident>>,
    vis: Option<SpannedKey<syn::Visibility>>,

    docs: Option<SpannedKey<Vec<syn::Attribute>>>,
}

impl FromMeta for GetterConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::Path(_) = meta {
            return Ok(Self::default());
        }

        // Reject empty parens such as `#[builder(getter())]`
        crate::parsing::require_non_empty_paren_meta_list_or_name_value(meta)?;

        // Nested `Parsed` struct used as a helper for parsing the verbose form
        #[derive(FromMeta)]
        struct Parsed {
            name: Option<SpannedKey<syn::Ident>>,
            vis: Option<SpannedKey<syn::Visibility>>,

            #[darling(rename = "doc", default, with = parse_docs, map = Some)]
            docs: Option<SpannedKey<Vec<syn::Attribute>>>,
        }

        let Parsed { name, vis, docs } = Parsed::from_meta(meta)?;

        Ok(Self { name, vis, docs })
    }
}

impl GetterConfig {
    pub(crate) fn name(&self) -> Option<&syn::Ident> {
        self.name.as_ref().map(|n| &n.value)
    }

    pub(crate) fn vis(&self) -> Option<&syn::Visibility> {
        self.vis.as_ref().map(|v| &v.value)
    }

    pub(crate) fn docs(&self) -> Option<&[syn::Attribute]> {
        self.docs.as_ref().map(|a| &a.value).map(|a| &**a)
    }
}

fn parse_docs(meta: &syn::Meta) -> Result<SpannedKey<Vec<syn::Attribute>>> {
    crate::parsing::parse_docs_without_self_mentions("builder struct's impl block", meta)
}
