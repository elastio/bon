use crate::parsing::SpannedKey;
use crate::util::prelude::*;
use darling::FromMeta;

const DOCS_CONTEXT: &str = "builder struct's impl block";

fn parse_docs(meta: &syn::Meta) -> Result<SpannedKey<Vec<syn::Attribute>>> {
    crate::parsing::parse_docs_without_self_mentions(DOCS_CONTEXT, meta)
}

#[derive(Debug)]
pub(crate) enum SetterFnName {
    Name(syn::Ident),
    Prefix(syn::Ident),
}

impl SetterFnName {
    fn new(
        name: Option<SpannedKey<syn::Ident>>,
        prefix: Option<SpannedKey<syn::Ident>>,
    ) -> Result<Option<SpannedKey<Self>>> {
        match (name, prefix) {
            (Some(name), None) => Ok(Some(name.map_value(SetterFnName::Name))),
            (None, Some(prefix)) => Ok(Some(prefix.map_value(SetterFnName::Prefix))),
            (None, None) => Ok(None),
            (Some(name), Some(prefix)) => {
                bail!(
                    &name.key,
                    "`{}` is mutually exclusive with `{}`",
                    name.key,
                    prefix.key,
                );
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct SettersConfig {
    pub(crate) name: Option<SpannedKey<SetterFnName>>,
    pub(crate) vis: Option<SpannedKey<syn::Visibility>>,
    pub(crate) docs: Option<SpannedKey<Vec<syn::Attribute>>>,
    pub(crate) fns: SettersFnsConfig,
}

impl FromMeta for SettersConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        #[derive(FromMeta)]
        struct Parsed {
            name: Option<SpannedKey<syn::Ident>>,
            prefix: Option<SpannedKey<syn::Ident>>,

            vis: Option<SpannedKey<syn::Visibility>>,

            #[darling(rename = "doc", default, map = Some, with = parse_docs)]
            docs: Option<SpannedKey<Vec<syn::Attribute>>>,

            #[darling(flatten)]
            fns: SettersFnsConfig,
        }

        let Parsed {
            name,
            prefix,
            vis,
            docs,
            fns,
        } = Parsed::from_meta(meta)?;

        Ok(SettersConfig {
            name: SetterFnName::new(name, prefix)?,
            vis,
            docs,
            fns,
        })
    }
}

#[derive(Debug, FromMeta)]
pub(crate) struct SettersFnsConfig {
    /// Config for the setter that accepts the value of type T for a member of
    /// type `Option<T>` or with `#[builder(default)]`.
    ///
    /// By default, it's named `{member}` without any prefix or suffix.
    pub(crate) some_fn: Option<SpannedKey<SetterFnSigConfig>>,

    /// The setter that accepts the value of type `Option<T>` for a member of
    /// type `Option<T>` or with `#[builder(default)]`.
    ///
    /// By default, it's named `maybe_{member}`.
    pub(crate) option_fn: Option<SpannedKey<SetterFnSigConfig>>,
}

#[derive(Debug, Default)]
pub(crate) struct SetterFnSigConfig {
    pub(crate) name: Option<SpannedKey<SetterFnName>>,
    pub(crate) vis: Option<SpannedKey<syn::Visibility>>,
    pub(crate) docs: Option<SpannedKey<Vec<syn::Attribute>>>,
}

impl FromMeta for SetterFnSigConfig {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::NameValue(meta) = meta {
            let val = &meta.value;
            let name = syn::parse2(val.to_token_stream())?;

            return Ok(SetterFnSigConfig {
                name: Some(SpannedKey::new(&meta.path, SetterFnName::Name(name))?),
                vis: None,
                docs: None,
            });
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: Option<SpannedKey<syn::Ident>>,
            prefix: Option<SpannedKey<syn::Ident>>,

            vis: Option<SpannedKey<syn::Visibility>>,

            #[darling(rename = "doc", default, map = Some, with = parse_docs)]
            docs: Option<SpannedKey<Vec<syn::Attribute>>>,
        }

        let Full {
            name,
            prefix,
            vis,
            docs,
        } = crate::parsing::parse_non_empty(meta)?;

        let config = SetterFnSigConfig {
            name: SetterFnName::new(name, prefix)?,
            vis,
            docs,
        };

        Ok(config)
    }
}
