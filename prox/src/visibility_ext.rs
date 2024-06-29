use crate::Result;
use easy_ext::ext;

#[ext(VisibilityExt)]
pub impl syn::Visibility {
    /// Returns [`syn::Visibility`] that is equivalent to the current visibility
    /// but for an item that is inside of the child module. This basically does
    /// the following conversions.
    ///
    /// - `pub` -> `pub` (unchanged)
    /// - `pub(crate)` -> `pub(crate)` (unchanged)
    /// - ` ` (default private visibility) -> `pub(super)`
    /// - `pub(super)` -> `pub(in super::super)`
    /// - `pub(in relative::path)` -> `pub(in super::relative::path)`
    /// - `pub(in ::absolute::path)` -> `pub(in ::absolute::path)` (unchanged)
    ///
    /// # Errors
    ///
    /// This function may return an error if it encounters some unexpected syntax.
    /// For example, some syntax that isn't known to the latest version of Rust
    /// this code was written for.
    fn into_equivalent_in_child_module(mut self) -> Result<syn::Visibility> {
        match &mut self {
            syn::Visibility::Public(_) => Ok(self),
            syn::Visibility::Inherited => Ok(syn::parse_quote!(pub(super))),
            syn::Visibility::Restricted(syn::VisRestricted {
                path,
                in_token: None,
                ..
            }) => {
                if path.is_ident("crate") {
                    return Ok(self);
                }

                if path.is_ident("super") {
                    return Ok(syn::parse_quote!(pub(in super::#path)));
                }

                crate::bail!(
                    &self,
                    "Expected either `crate` or `super` or `in some::path` inside of \
                    `pub(...)` but got something else. This may be because a new \
                    syntax for visibility was released in a newer Rust version, \
                    but this crate doesn't support it."
                );
            }
            syn::Visibility::Restricted(syn::VisRestricted {
                path,
                in_token: Some(_),
                ..
            }) => {
                if path.leading_colon.is_some() {
                    return Ok(self);
                }

                path.segments.insert(0, syn::parse_quote!(super));

                Ok(self)
            }
        }
    }
}
