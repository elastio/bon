use crate::{
    parsing::{ItemSigConfig, ItemSigConfigParsing},
    util::prelude::Result as BonResult,
};
use darling::FromMeta;

use super::{
    docs_utils::{parse_docs, DOCS_CONTEXT},
    SpannedKey,
};

// fn parse_getter_fn(meta: &syn::Meta) -> BonResult<SpannedKey<ItemSigConfig>> {
//     let params = ItemSigConfigParsing {
//         meta,
//         reject_self_mentions: Some(DOCS_CONTEXT),
//     }
//     .parse()?;

//     SpannedKey::new(meta.path(), params)
// }

#[derive(Debug, FromMeta)]
pub(crate) struct GetterConfig {
    pub(crate) name: Option<SpannedKey<syn::Ident>>,
    pub(crate) vis: Option<SpannedKey<syn::Visibility>>,

    #[darling(rename = "doc", default, with = parse_docs, map = Some)]
    pub(crate) docs: Option<SpannedKey<Vec<syn::Attribute>>>,
}
