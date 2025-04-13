use crate::util::prelude::*;
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

        if let Some(const_) = &self.config.const_ {
            if self.fn_item.orig.sig.constness.is_none() {
                bail!(
                    &const_,
                    "#[builder(const)] requires the underlying function to be \
                    marked as `const fn`"
                );
            }
        }

        Ok(())
    }

    pub(crate) fn warnings(&self) -> TokenStream {
        // We used to emit some warnings here previously, but then that logic
        // was removed. The code for the `warnings()` method was preserved just
        // in case if we need to issue some new warnings again. However, it's not
        // critical. Feel free to eliminate this method if you feel like it.
        let _ = self;

        TokenStream::new()
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
