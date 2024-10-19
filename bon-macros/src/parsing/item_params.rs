use super::SpannedKey;
use crate::util::prelude::*;
use darling::FromMeta;

#[derive(Debug, Clone, Default)]
pub(crate) struct ItemParams {
    pub(crate) name: Option<SpannedKey<syn::Ident>>,
    pub(crate) vis: Option<SpannedKey<syn::Visibility>>,
    pub(crate) docs: Option<SpannedKey<Vec<syn::Attribute>>>,
}

impl ItemParams {
    pub(crate) fn name(&self) -> Option<&syn::Ident> {
        self.name.as_ref().map(|name| &name.value)
    }

    pub(crate) fn vis(&self) -> Option<&syn::Visibility> {
        self.vis.as_ref().map(|vis| &vis.value)
    }

    pub(crate) fn docs(&self) -> Option<&[syn::Attribute]> {
        self.docs.as_ref().map(|docs| docs.value.as_slice())
    }
}

pub(crate) struct ItemParamsParsing<'a> {
    pub(crate) meta: &'a syn::Meta,
    pub(crate) reject_self_mentions: Option<&'static str>,
}

impl ItemParamsParsing<'_> {
    pub(crate) fn parse(self) -> Result<ItemParams> {
        let meta = self.meta;

        if let syn::Meta::NameValue(meta) = meta {
            let val = &meta.value;
            let name = syn::parse2(val.to_token_stream())?;

            return Ok(ItemParams {
                name: Some(SpannedKey::new(&meta.path, name)?),
                vis: None,
                docs: None,
            });
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: Option<SpannedKey<syn::Ident>>,
            vis: Option<SpannedKey<syn::Visibility>>,

            #[darling(default, with = super::parse_docs, map = Some)]
            doc: Option<SpannedKey<Vec<syn::Attribute>>>,
        }

        let full: Full = crate::parsing::parse_non_empty_paren_meta_list(meta)?;

        if let Some(context) = self.reject_self_mentions {
            if let Some(docs) = &full.doc {
                crate::parsing::reject_self_mentions_in_docs(context, docs)?;
            }
        }

        let params = ItemParams {
            name: full.name,
            vis: full.vis,
            docs: full.doc,
        };

        Ok(params)
    }
}
