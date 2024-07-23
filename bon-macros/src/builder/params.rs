use crate::util::prelude::*;
use darling::FromMeta;
use quote::quote;

#[derive(Debug, FromMeta)]
pub(crate) struct BuilderParams {
    pub(crate) finish_fn: Option<syn::Ident>,
    pub(crate) builder_type: Option<syn::Ident>,
}

#[derive(Debug, Default)]
pub(crate) struct ItemParams {
    pub(crate) name: Option<syn::Ident>,
    pub(crate) vis: Option<syn::Visibility>,
}

impl FromMeta for ItemParams {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::NameValue(meta) = meta {
            let val = &meta.value;
            let name = syn::parse2(quote!(#val))?;

            return Ok(Self {
                name: Some(name),
                vis: None,
            });
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: Option<syn::Ident>,
            vis: Option<syn::Visibility>,
        }

        let full = Full::from_meta(meta)?;

        let is_empty = matches!(
            full,
            Full {
                name: None,
                vis: None,
            }
        );

        if is_empty {
            bail!(meta, "expected at least one parameter in parentheses");
        }

        let me = Self {
            name: full.name,
            vis: full.vis,
        };

        Ok(me)
    }
}
