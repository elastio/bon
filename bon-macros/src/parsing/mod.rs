mod docs;
mod item_params;
mod simple_closure;
mod spanned_key;

pub(crate) use docs::*;
pub(crate) use item_params::*;
pub(crate) use simple_closure::*;
pub(crate) use spanned_key::*;

use crate::util::prelude::*;
use darling::FromMeta;

pub(crate) fn require_non_empty_paren_meta_list<T: FromMeta>(meta: &syn::Meta) -> Result<T> {
    if let syn::Meta::List(meta) = meta {
        meta.require_parens_delim()?;
        if meta.tokens.is_empty() {
            bail!(meta, "expected at least one parameter in parentheses");
        }
    }

    T::from_meta(meta)
}
