use std::ops::Deref;

use darling::FromMeta;
use proc_macro2::Span;
use syn::spanned::Spanned;

use super::{docs_utils::parse_docs, SpannedKey};

/// Wrapper around the getter config that allows it to work as both a flag and
/// a config.
#[derive(Debug)]
pub(crate) struct OptionalGetterConfig {
    getter_config: Option<GetterConfig>,
}

impl FromMeta for OptionalGetterConfig {
    fn from_none() -> Option<Self> {
        Some(Self {
            getter_config: None,
        })
    }

    fn from_meta(mi: &syn::Meta) -> darling::Result<Self> {
        GetterConfig::from_meta(mi).map(|getter_config| Self {
            getter_config: Some(getter_config),
        })
    }
}

impl Deref for OptionalGetterConfig {
    type Target = Option<GetterConfig>;

    fn deref(&self) -> &Self::Target {
        &self.getter_config
    }
}

#[derive(Debug)]
pub(crate) enum GetterConfig {
    #[allow(unused)]
    Inferred(Span),
    Specified(SpecifiedGetterConfig),
}

impl FromMeta for GetterConfig {
    fn from_none() -> Option<Self> {
        None
    }

    fn from_meta(mi: &syn::Meta) -> darling::Result<Self> {
        if let syn::Meta::Path(p) = mi {
            Ok(Self::Inferred(p.span()))
        } else {
            SpecifiedGetterConfig::from_meta(mi).map(Self::Specified)
        }
    }
}

impl GetterConfig {
    pub(crate) fn name(&self) -> Option<&syn::Ident> {
        match self {
            Self::Inferred(_) => None,
            Self::Specified(config) => config.name.as_ref().map(|n| &n.value),
        }
    }

    pub(crate) fn vis(&self) -> Option<&syn::Visibility> {
        match self {
            Self::Inferred(_) => None,
            Self::Specified(config) => config.vis.as_ref().map(|v| &v.value),
        }
    }

    pub(crate) fn docs(&self) -> Option<&[syn::Attribute]> {
        match self {
            Self::Inferred(_) => None,
            Self::Specified(config) => config.docs.as_ref().map(|a| &a.value).map(|a| &**a),
        }
    }
}

#[derive(Debug, FromMeta)]
pub(crate) struct SpecifiedGetterConfig {
    name: Option<SpannedKey<syn::Ident>>,
    vis: Option<SpannedKey<syn::Visibility>>,

    #[darling(rename = "doc", default, with = parse_docs, map = Some)]
    docs: Option<SpannedKey<Vec<syn::Attribute>>>,
}
