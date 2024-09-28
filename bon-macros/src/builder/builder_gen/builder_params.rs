use crate::util::prelude::*;
use darling::FromMeta;
use proc_macro2::Span;
use quote::quote;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::visit::Visit;

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

    #[darling(multiple)]
    pub(crate) on: Vec<OnParams>,

    /// Specifies the derives to apply to the builder.
    #[darling(default)]
    pub(crate) derive: BuilderDerives,
}

#[derive(Debug, Clone, Default, FromMeta)]
pub(crate) struct BuilderDerives {
    #[darling(rename = "Clone")]
    pub(crate) clone: darling::util::Flag,

    #[darling(rename = "Debug")]
    pub(crate) debug: darling::util::Flag,
}

#[derive(Debug)]
pub(crate) struct OnParams {
    pub(crate) type_pattern: syn::Type,
    pub(crate) into: darling::util::Flag,
    pub(crate) overwritable: darling::util::Flag,
}

impl Parse for OnParams {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let type_pattern = input.parse()?;

        let _ = input.parse::<syn::Token![,]>()?;
        let rest: TokenStream2 = input.parse()?;

        #[derive(FromMeta)]
        struct Parsed {
            into: darling::util::Flag,
            overwritable: darling::util::Flag,
        }

        let parsed = Parsed::from_meta(&syn::parse_quote!(on(#rest)))?;

        {
            // Validate that at least some option was enabled.
            // This lives in a separate block to make sure that if a new
            // field is added to `Parsed` and unused here, then a compiler
            // warning is emitted.
            let Parsed { into, overwritable } = &parsed;

            if !into.is_present() && !overwritable.is_present() {
                return Err(syn::Error::new_spanned(
                    &rest,
                    "this #[builder(on(type_pattern, ...))] contains no options to override \
                    the default behavior for the selected setters like `into`, so it \
                    does nothing",
                ));
            }
        }

        struct FindAttr {
            attr: Option<Span>,
        }

        impl Visit<'_> for FindAttr {
            fn visit_attribute(&mut self, attr: &'_ syn::Attribute) {
                self.attr.get_or_insert(attr.span());
            }
        }

        let mut find_attr = FindAttr { attr: None };
        find_attr.visit_type(&type_pattern);
        let attr_in_type_pattern = find_attr.attr;

        if let Some(attr) = attr_in_type_pattern {
            return Err(syn::Error::new(
                attr,
                "nested attributes are not allowed in the type pattern of \
                #[builder(on(type_pattern, ...))]",
            ));
        }

        // Validate that the pattern. The validation is done in the process
        // of matching the types. To make sure that matching traverses the
        // full pattern we match it with itself.
        let type_pattern_matches_itself = type_pattern.matches(&type_pattern)?;

        assert!(
            type_pattern_matches_itself,
            "BUG: the type pattern does not match itself: {type_pattern:#?}"
        );

        let Parsed { into, overwritable } = parsed;

        Ok(Self {
            type_pattern,
            into,
            overwritable,
        })
    }
}

impl FromMeta for OnParams {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        let meta = match meta {
            syn::Meta::List(meta) => meta,
            _ => bail!(
                meta,
                "Expected an attribute of form `on(type_pattern, ...)`"
            ),
        };

        let me = syn::parse2(meta.tokens.clone())?;

        Ok(me)
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ItemParams {
    pub(crate) name: Option<syn::Ident>,
    pub(crate) vis: Option<syn::Visibility>,
    pub(crate) docs: Option<Vec<syn::Attribute>>,
}

pub(crate) struct ItemParamsParsing<'a> {
    pub(crate) meta: &'a syn::Meta,
    pub(crate) reject_self_mentions: Option<&'static str>,
}

impl ItemParamsParsing<'_> {
    pub(crate) fn parse(self) -> Result<ItemParams> {
        let params = Self::params_from_meta(self.meta)?;

        if let Some(context) = self.reject_self_mentions {
            if let Some(docs) = &params.docs {
                super::reject_self_mentions_in_docs(context, docs)?;
            }
        }

        Ok(params)
    }

    fn params_from_meta(meta: &syn::Meta) -> Result<ItemParams> {
        if let syn::Meta::NameValue(meta) = meta {
            let val = &meta.value;
            let name = syn::parse2(quote!(#val))?;

            return Ok(ItemParams {
                name: Some(name),
                vis: None,
                docs: None,
            });
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: Option<syn::Ident>,
            vis: Option<syn::Visibility>,
            docs: Option<syn::Meta>,
        }

        let full = Full::from_meta(meta)?;

        let is_empty = matches!(
            full,
            Full {
                name: None,
                vis: None,
                docs: None,
            }
        );

        if is_empty {
            bail!(meta, "expected at least one parameter in parentheses");
        }

        let docs = full
            .docs
            .map(|docs| {
                let docs = docs.require_list()?;
                let docs = docs.parse_args_with(syn::Attribute::parse_outer)?;

                for attr in &docs {
                    if !attr.is_doc() {
                        bail!(attr, "expected a doc comment");
                    }
                }

                Ok(docs)
            })
            .transpose()?;

        let params = ItemParams {
            name: full.name,
            vis: full.vis,
            docs,
        };

        Ok(params)
    }
}
