mod on_params;

pub(crate) use on_params::OnParams;

use crate::parsing::{ItemParams, ItemParamsParsing};
use crate::util::prelude::*;
use darling::FromMeta;
use syn::punctuated::Punctuated;

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

    #[darling(multiple, with = crate::parsing::parse_non_empty_paren_meta_list)]
    pub(crate) on: Vec<OnParams>,

    /// Specifies the derives to apply to the builder.
    #[darling(default, with = crate::parsing::parse_non_empty_paren_meta_list)]
    pub(crate) derive: BuilderDerives,
}

#[derive(Debug, Clone, Default, FromMeta)]
pub(crate) struct BuilderDerives {
    #[darling(rename = "Clone")]
    pub(crate) clone: Option<BuilderDerive>,

    #[darling(rename = "Debug")]
    pub(crate) debug: Option<BuilderDerive>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct BuilderDerive {
    pub(crate) bounds: Option<Punctuated<syn::WherePredicate, syn::Token![,]>>,
}

impl FromMeta for BuilderDerive {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::Path(_) = meta {
            return Ok(Self { bounds: None });
        }

        #[derive(FromMeta)]
        struct Parsed {
            #[darling(with = crate::parsing::parse_paren_meta_list_with_terminated)]
            bounds: Punctuated<syn::WherePredicate, syn::Token![,]>,
        }

        meta.require_list()?.require_parens_delim()?;

        let Parsed { bounds } = Parsed::from_meta(meta)?;

        Ok(Self {
            bounds: Some(bounds),
        })
    }
}
