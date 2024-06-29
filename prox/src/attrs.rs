use easy_ext::ext;
use syn::punctuated::Punctuated;

#[ext(AttributeExt)]
pub impl syn::Attribute {
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

#[ext(MetaListExt)]
pub impl syn::MetaList {
    /// Parse the comma-separated list of [`syn::Meta`] in the list
    fn items(&self) -> Punctuated<syn::Meta, syn::Token![,]> {
        self.parse_args_with(Punctuated::parse_terminated)
            .unwrap_or_default()
    }
}

/// Returns the [`syn::Attribute`] that matches the given [`syn::Meta`] attribute
pub fn find_attr<'hay>(
    haystack: &'hay [syn::Attribute],
    needle: &syn::Meta,
) -> Option<&'hay syn::Attribute> {
    haystack.iter().find(|attr| attr.meta == *needle)
}

/// Checks if the given [`syn::Meta`] attribute is present in the haystack
pub fn contains_attr(haystack: &[syn::Attribute], needle: &syn::Meta) -> bool {
    find_attr(haystack, needle).is_some()
}

/// Prepends the given [`syn::Meta`] attribute to the list of attrs if it's not present there
pub fn prepend_attr_if_absent(attrs: &mut Vec<syn::Attribute>, insert_meta: syn::Meta) {
    if !contains_attr(attrs, &insert_meta) {
        attrs.insert(0, syn::parse_quote!(#[#insert_meta]));
    }
}
