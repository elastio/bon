use darling::ast::NestedMeta;
use darling::FromMeta;
use prox::prelude::*;

#[derive(Debug, FromMeta)]
pub(crate) struct BuilderParams {
    pub(crate) finish_fn: Option<syn::Ident>,
    pub(crate) builder_type: Option<syn::Ident>,
}

#[derive(Debug)]
pub(crate) struct ItemParams {
    pub(crate) name: Option<syn::Ident>,
    pub(crate) vis: Option<syn::Visibility>,
}

impl FromMeta for ItemParams {
    fn from_nested_meta(item: &NestedMeta) -> Result<Self> {
        let me = match item {
            NestedMeta::Lit(lit) => Self {
                name: Some(syn::Ident::from_value(lit)?),
                vis: None,
            },
            NestedMeta::Meta(ref mi) => {
                #[derive(Debug, FromMeta)]
                struct Full {
                    name: Option<syn::Ident>,
                    vis: Option<syn::Visibility>,
                }

                let full = Full::from_meta(mi)?;
                Self {
                    name: full.name,
                    vis: full.vis,
                }
            }
        };

        Ok(me)
    }
}
