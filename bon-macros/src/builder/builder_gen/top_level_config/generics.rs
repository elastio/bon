use crate::parsing::{ItemSigConfig, ItemSigConfigParsing};
use crate::util::prelude::*;
use darling::FromMeta;

#[derive(Debug, Clone, Default)]
pub(crate) struct GenericsConfig {
    pub(crate) setters: Option<ItemSigConfig<String>>,
}

impl FromMeta for GenericsConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        meta.require_list()?.require_parens_delim()?;

        #[derive(FromMeta)]
        struct Parsed {
            #[darling(default, with = parse_setters_config, map = Some)]
            setters: Option<ItemSigConfig<String>>,
        }

        let parsed = Parsed::from_meta(meta)?;

        Ok(Self {
            setters: parsed.setters,
        })
    }
}

fn parse_setters_config(meta: &syn::Meta) -> Result<ItemSigConfig<String>> {
    if !cfg!(feature = "experimental-generics-setters") {
        bail!(
            meta,
            "🔬 `generics(setters(...))` attribute is experimental and requires \
             \"experimental-generics-setters\" cargo feature to be enabled",
        );
    }

    const DOCS_CONTEXT: &str = "builder struct's impl block";
    ItemSigConfigParsing::new(meta, Some(DOCS_CONTEXT)).parse()
}
