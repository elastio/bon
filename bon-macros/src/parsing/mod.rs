mod docs;
mod item_params;

pub(crate) use docs::*;
pub(crate) use item_params::*;

use crate::util::prelude::*;
use darling::FromMeta;

pub(crate) fn require_paren_delim_for_meta_list<T: FromMeta>(meta: &syn::Meta) -> Result<T> {
    if let syn::Meta::List(meta) = meta {
        meta.require_parens_delim()?;
    }

    T::from_meta(meta)
}
