use crate::util::prelude::*;
use syn::spanned::Spanned;
use syn::visit::Visit;

impl super::FnInputCtx<'_> {
    pub(super) fn validate(&self) -> Result {
        if self.impl_ctx.is_none() {
            let explanation = "\
                but #[bon] attribute is absent on top of the impl block; this \
                additional #[bon] attribute on the impl block is required for \
                the macro to see the type of `Self` and properly generate \
                the builder struct definition adjacently to the impl block.";

            if let Some(receiver) = &self.fn_item.orig.sig.receiver() {
                bail!(
                    &receiver.self_token,
                    "function contains a `self` parameter {explanation}"
                );
            }

            let mut ctx = FindSelfReference::default();
            ctx.visit_item_fn(&self.fn_item.orig);
            if let Some(self_span) = ctx.self_span {
                bail!(
                    &self_span,
                    "function contains a `Self` type reference {explanation}"
                );
            }
        }

        Ok(())
    }

    pub(crate) fn warnings(&self) -> TokenStream {
        let mut warnings = TokenStream::new();

        let bon = self
            .config
            .bon
            .clone()
            .unwrap_or_else(|| syn::parse_quote!(::bon));

        let instrument = self
            .fn_item
            .orig
            .attrs
            .iter()
            .filter_map(|attr| attr.path().segments.last())
            .find(|segment| segment.ident == "instrument");

        if let Some(instrument) = instrument {
            let span = instrument.span();

            warnings.extend(quote_spanned! {span=>
                use #bon::__::warnings::tracing_instrument_attribute_after_builder as _;
            });
        }

        warnings
    }
}

#[derive(Default)]
struct FindSelfReference {
    self_span: Option<Span>,
}

impl Visit<'_> for FindSelfReference {
    fn visit_item(&mut self, _: &syn::Item) {
        // Don't recurse into nested items. We are interested in the reference
        // to `Self` on the current item level
    }

    fn visit_path(&mut self, path: &syn::Path) {
        if self.self_span.is_some() {
            return;
        }
        syn::visit::visit_path(self, path);

        let first_segment = match path.segments.first() {
            Some(first_segment) => first_segment,
            _ => return,
        };

        if first_segment.ident == "Self" {
            self.self_span = Some(first_segment.ident.span());
        }
    }
}
