use easy_ext::ext;

#[ext(AttributeExt)]
pub(crate) impl syn::Attribute {
    /// Returns `true` if the attribute represents a `#[doc = ...]` attribute.
    fn is_doc(&self) -> bool {
        self.as_doc().is_some()
    }

    /// Checks if the attribute represents a `#[doc = ...]` attribute. If so,
    /// returns the expression that represents the documentation value.
    fn as_doc(&self) -> Option<&syn::Expr> {
        let syn::Meta::NameValue(attr) = &self.meta else {
            return None;
        };

        if !attr.path.is_ident("doc") {
            return None;
        }

        Some(&attr.value)
    }
}
