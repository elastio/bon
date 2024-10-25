mod on;

pub(crate) use on::OnConfig;

use crate::parsing::{ItemSigConfig, ItemSigConfigParsing, SpannedKey};
use crate::util::prelude::*;
use darling::FromMeta;
use syn::punctuated::Punctuated;

fn parse_finish_fn(meta: &syn::Meta) -> Result<ItemSigConfig> {
    ItemSigConfigParsing {
        meta,
        reject_self_mentions: Some("builder struct's impl block"),
    }
    .parse()
}

fn parse_builder_type(meta: &syn::Meta) -> Result<ItemSigConfig> {
    ItemSigConfigParsing {
        meta,
        reject_self_mentions: Some("builder struct"),
    }
    .parse()
}

fn parse_state_mod(meta: &syn::Meta) -> Result<ItemSigConfig> {
    ItemSigConfigParsing {
        meta,
        reject_self_mentions: Some("builder's state module"),
    }
    .parse()
}

fn parse_start_fn(meta: &syn::Meta) -> Result<ItemSigConfig> {
    ItemSigConfigParsing {
        meta,
        reject_self_mentions: None,
    }
    .parse()
}

#[derive(Debug, FromMeta)]
pub(crate) struct TopLevelConfig {
    #[darling(rename = "crate", default, map = Some, with = crate::parsing::parse_bon_crate_path)]
    pub(crate) bon: Option<syn::Path>,

    #[darling(default, with = parse_start_fn)]
    pub(crate) start_fn: ItemSigConfig,

    #[darling(default, with = parse_finish_fn)]
    pub(crate) finish_fn: ItemSigConfig,

    #[darling(default, with = parse_builder_type)]
    pub(crate) builder_type: ItemSigConfig,

    #[darling(default, with = parse_state_mod)]
    pub(crate) state_mod: ItemSigConfig,

    #[darling(multiple, with = crate::parsing::parse_non_empty_paren_meta_list)]
    pub(crate) on: Vec<OnConfig>,

    /// Specifies the derives to apply to the builder.
    #[darling(default, with = crate::parsing::parse_non_empty_paren_meta_list)]
    pub(crate) derive: DerivesConfig,
}

impl TopLevelConfig {
    pub(crate) fn parse_for_fn(meta_list: &[darling::ast::NestedMeta]) -> Result<Self> {
        let me = Self::from_list(meta_list)?;

        if me.start_fn.name.is_none() {
            let ItemSigConfig { name: _, vis, docs } = &me.start_fn;

            let unexpected_param = None
                .or_else(|| vis.as_ref().map(SpannedKey::key))
                .or_else(|| docs.as_ref().map(SpannedKey::key));

            if let Some(unexpected_param) = unexpected_param {
                bail!(
                    unexpected_param,
                    "#[builder(start_fn({unexpected_param}))] requires that you \
                    also specify #[builder(start_fn(name))] which makes the starting \
                    function not to replace the positional function under the #[builder] \
                    attribute; by default (without the explicit #[builder(start_fn(name))]) \
                    the name, visibility and documentation of the positional \
                    function are all copied to the starting function, and the positional \
                    function under the #[builder] attribute becomes private with \
                    #[doc(hidden)] and it's renamed (the name is not guaranteed \
                    to be stable) to make it inaccessible even within the current module",
                );
            }
        }

        Ok(me)
    }
}

#[derive(Debug, Clone, Default, FromMeta)]
pub(crate) struct DerivesConfig {
    #[darling(rename = "Clone")]
    pub(crate) clone: Option<DeriveConfig>,

    #[darling(rename = "Debug")]
    pub(crate) debug: Option<DeriveConfig>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct DeriveConfig {
    pub(crate) bounds: Option<Punctuated<syn::WherePredicate, syn::Token![,]>>,
}

impl FromMeta for DeriveConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::Path(_) = meta {
            return Ok(Self { bounds: None });
        }

        meta.require_list()?.require_parens_delim()?;

        #[derive(FromMeta)]
        struct Parsed {
            #[darling(with = crate::parsing::parse_paren_meta_list_with_terminated)]
            bounds: Punctuated<syn::WherePredicate, syn::Token![,]>,
        }

        let Parsed { bounds } = Parsed::from_meta(meta)?;

        Ok(Self {
            bounds: Some(bounds),
        })
    }
}
