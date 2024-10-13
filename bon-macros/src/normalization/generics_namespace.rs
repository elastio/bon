use std::collections::BTreeSet;

#[derive(Default, Clone)]
pub(crate) struct GenericsNamespace {
    /// Set of identifiers referenced in the syntax element.
    pub(crate) idents: BTreeSet<String>,

    /// Set of lifetimes referenced in the syntax element.
    pub(crate) lifetimes: BTreeSet<String>,
}

impl syn::visit::Visit<'_> for GenericsNamespace {
    fn visit_ident(&mut self, ident: &syn::Ident) {
        self.idents.insert(ident.to_string());
    }

    fn visit_lifetime(&mut self, lifetime: &syn::Lifetime) {
        self.lifetimes.insert(lifetime.ident.to_string());
    }

    fn visit_item(&mut self, _item: &syn::Item) {
        // Don't recurse into child items. They don't inherit the parent item's generics.
    }
}
