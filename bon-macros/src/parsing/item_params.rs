use crate::util::prelude::*;
use darling::FromMeta;

#[derive(Debug, Clone, Default)]
pub(crate) struct ItemParams {
    pub(crate) name: Option<syn::Ident>,
    pub(crate) vis: Option<syn::Visibility>,
    pub(crate) docs: Option<Vec<syn::Attribute>>,
}

pub(crate) struct ItemParamsParsing<'a> {
    pub(crate) meta: &'a syn::Meta,
    pub(crate) reject_self_mentions: Option<&'static str>,
}

impl ItemParamsParsing<'_> {
    pub(crate) fn parse(self) -> Result<ItemParams> {
        let params = Self::params_from_meta(self.meta)?;

        if let Some(context) = self.reject_self_mentions {
            if let Some(docs) = &params.docs {
                crate::parsing::reject_self_mentions_in_docs(context, docs)?;
            }
        }

        Ok(params)
    }

    fn params_from_meta(meta: &syn::Meta) -> Result<ItemParams> {
        if let syn::Meta::NameValue(meta) = meta {
            let val = &meta.value;
            let name = syn::parse2(quote!(#val))?;

            return Ok(ItemParams {
                name: Some(name),
                vis: None,
                docs: None,
            });
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: Option<syn::Ident>,
            vis: Option<syn::Visibility>,
            docs: Option<syn::Meta>,
        }

        let full = crate::parsing::require_paren_delim_for_meta_list(meta)?;

        let is_empty = matches!(
            full,
            Full {
                name: None,
                vis: None,
                docs: None,
            }
        );

        if is_empty {
            bail!(meta, "expected at least one parameter in parentheses");
        }

        let docs = full.docs.as_ref().map(super::parse_docs).transpose()?;

        let params = ItemParams {
            name: full.name,
            vis: full.vis,
            docs,
        };

        Ok(params)
    }
}
