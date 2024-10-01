use crate::util::prelude::*;
use darling::FromMeta;

#[derive(Debug)]
pub(crate) struct SetterParams {
    pub(crate) name: Option<syn::Ident>,
    pub(crate) vis: Option<syn::Visibility>,
    pub(crate) docs: Option<Vec<syn::Attribute>>,
    pub(crate) with: Option<syn::Expr>,
}

impl FromMeta for SetterParams {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        let err = || {
            err!(
                meta,
                "expected a #[builder(setter = {{closure}})], or \
                #[buidler(setter({{key}} = {{value}}... ))] syntax"
            )
        };

        let meta_list = match meta {
            syn::Meta::List(meta) => meta,
            syn::Meta::Path(_) => return Err(err()),
            syn::Meta::NameValue(meta) => match &meta.value {
                syn::Expr::Closure(closure) => {
                    return Ok(Self {
                        name: None,
                        vis: None,
                        docs: None,
                        with: Some(syn::Expr::Closure(closure.clone())),
                    })
                },
                _ => return Err(err()),
            },
        };

        #[derive(FromMeta)]
        struct Parsed {
            name: Option<syn::Ident>,
            vis: Option<syn::Visibility>,

            #[darling(default, with = crate::util::parsing::parse_docs, map = Some)]
            docs: Option<Vec<syn::Attribute>>,
        }

        let Parsed { name, vis, docs } = Parsed::from_meta(meta)?;

        if let Some(docs) = &docs {
            crate::util::parsing::reject_self_mentions_in_docs(
                "builder struct's impl block",
                docs,
            )?;
        }

        Ok(Self { name, vis, docs })

        match meta_list.delimiter {
            syn::MacroDelimiter::Bracket(_) => return Err(err()),
            syn::MacroDelimiter::Paren(_) => {
                return SetterParenParams::from_meta(meta).map(Self::Paren)
            }
            syn::MacroDelimiter::Brace(_) => {

            },
        }
    }
}
