mod on_params;

pub(crate) use on_params::OnParams;

use crate::parsing::{ItemParams, ItemParamsParsing};
use crate::util::prelude::*;
use darling::FromMeta;

fn parse_finish_fn(meta: &syn::Meta) -> Result<ItemParams> {
    ItemParamsParsing {
        meta,
        reject_self_mentions: Some("builder struct's impl block"),
    }
    .parse()
}

fn parse_builder_type(meta: &syn::Meta) -> Result<ItemParams> {
    ItemParamsParsing {
        meta,
        reject_self_mentions: Some("builder struct"),
    }
    .parse()
}

fn parse_state_mod(meta: &syn::Meta) -> Result<ItemParams> {
    ItemParamsParsing {
        meta,
        reject_self_mentions: Some("builder module"),
    }
    .parse()
}

#[derive(Debug, FromMeta)]
pub(crate) struct BuilderParams {
    #[darling(default, with = parse_finish_fn)]
    pub(crate) finish_fn: ItemParams,

    #[darling(default, with = parse_builder_type)]
    pub(crate) builder_type: ItemParams,

    #[darling(default, with = parse_state_mod)]
    pub(crate) state_mod: ItemParams,

    #[darling(multiple, with = crate::parsing::require_non_empty_paren_meta_list)]
    pub(crate) on: Vec<OnParams>,

    /// Specifies the derives to apply to the builder.
    #[darling(default, with = crate::parsing::require_non_empty_paren_meta_list)]
    pub(crate) derive: BuilderDerives,
}

#[derive(Debug, Clone, Default, FromMeta)]
pub(crate) struct BuilderDerives {
    #[darling(rename = "Clone")]
    pub(crate) clone: darling::util::Flag,

    #[darling(rename = "Debug")]
    pub(crate) debug: darling::util::Flag,
}
