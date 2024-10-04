use crate::util::prelude::*;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::visit::Visit;
use darling::FromMeta;

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
        let rest: TokenStream = input.parse()?;

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
