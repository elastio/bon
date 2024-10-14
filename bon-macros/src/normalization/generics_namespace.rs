use crate::util::prelude::*;
use proc_macro2::TokenTree;
use std::collections::BTreeSet;
use syn::visit::Visit;

#[derive(Debug, Default, Clone)]
pub(crate) struct GenericsNamespace {
    /// Set of identifiers referenced in the syntax element.
    pub(crate) idents: BTreeSet<String>,

    /// Set of lifetimes referenced in the syntax element.
    pub(crate) lifetimes: BTreeSet<String>,
}

impl Visit<'_> for GenericsNamespace {
    fn visit_ident(&mut self, ident: &syn::Ident) {
        self.idents.insert(ident.to_string());
    }

    fn visit_meta_list(&mut self, meta_list: &syn::MetaList) {
        syn::visit::visit_meta_list(self, meta_list);
        self.visit_token_stream(meta_list.tokens.clone());
    }

    fn visit_lifetime(&mut self, lifetime: &syn::Lifetime) {
        self.lifetimes.insert(lifetime.ident.to_string());
    }

    fn visit_item(&mut self, _item: &syn::Item) {
        // Don't recurse into child items. They don't inherit the parent item's generics.
    }
}

impl GenericsNamespace {
    pub(crate) fn visit_token_stream(&mut self, token_stream: TokenStream) {
        let mut tokens = token_stream.into_iter().peekable();
        while let Some(tt) = tokens.next() {
            match tt {
                TokenTree::Group(group) => {
                    self.visit_token_stream(group.stream());
                }
                TokenTree::Ident(ident) => {
                    self.visit_ident(&ident);
                }
                TokenTree::Punct(punct) => {
                    if punct.as_char() != '\'' {
                        continue;
                    }
                    if let Some(TokenTree::Ident(ident)) = tokens.peek() {
                        self.lifetimes.insert(ident.to_string());
                        tokens.next();
                    }
                }
                TokenTree::Literal(_) => {}
            }
        }
    }
}
