use crate::util::prelude::*;
use darling::FromMeta;
use proc_macro2::Span;
use quote::quote;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::visit::Visit;

#[derive(Debug, FromMeta)]
pub(crate) struct BuilderParams {
    pub(crate) finish_fn: Option<syn::Ident>,
    pub(crate) builder_type: Option<syn::Ident>,

    #[darling(multiple)]
    pub(crate) on: Vec<ConditionalParams>,

    /// Specifies the derives to apply to the builder.
    pub(crate) derive: Option<BuilderDerives>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct BuilderDerives {
    #[darling(rename = "Clone")]
    pub(crate) clone: darling::util::Flag,

    #[darling(rename = "Debug")]
    pub(crate) debug: darling::util::Flag,
}

#[derive(Debug)]
pub(crate) struct ConditionalParams {
    pub(crate) type_pattern: syn::Type,
    pub(crate) into: darling::util::Flag,
}

impl Parse for ConditionalParams {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let type_pattern = input.parse()?;

        let _ = input.parse::<syn::Token![,]>()?;
        let rest: TokenStream2 = input.parse()?;

        #[derive(FromMeta)]
        struct Parsed {
            into: darling::util::Flag,
        }

        let Parsed { into } = Parsed::from_meta(&syn::parse_quote!(on(#rest)))?;

        if !into.is_present() {
            return Err(syn::Error::new_spanned(
                &rest,
                "this #[builder(on(type_pattern, ...))] contains no options to override \
                the default behavior for the selected setters like `into`, so it \
                does nothing",
            ));
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

        Ok(Self { type_pattern, into })
    }
}

impl FromMeta for ConditionalParams {
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

#[derive(Debug, Default)]
pub(crate) struct ItemParams {
    pub(crate) name: Option<syn::Ident>,
    pub(crate) vis: Option<syn::Visibility>,
}

impl FromMeta for ItemParams {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        if let syn::Meta::NameValue(meta) = meta {
            let val = &meta.value;
            let name = syn::parse2(quote!(#val))?;

            return Ok(Self {
                name: Some(name),
                vis: None,
            });
        }

        #[derive(Debug, FromMeta)]
        struct Full {
            name: Option<syn::Ident>,
            vis: Option<syn::Visibility>,
        }

        let full = Full::from_meta(meta)?;

        let is_empty = matches!(
            full,
            Full {
                name: None,
                vis: None,
            }
        );

        if is_empty {
            bail!(meta, "expected at least one parameter in parentheses");
        }

        let me = Self {
            name: full.name,
            vis: full.vis,
        };

        Ok(me)
    }
}
