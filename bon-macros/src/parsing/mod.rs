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

pub(crate) fn parse_non_empty_paren_meta_list<T: FromMeta>(meta: &syn::Meta) -> Result<T> {
    require_non_empty_paren_meta_list(meta)?;
    T::from_meta(meta)
}

pub(crate) fn require_non_empty_paren_meta_list(meta: &syn::Meta) -> Result {
    match meta {
        syn::Meta::List(meta) => {
            meta.require_parens_delim()?;

            if meta.tokens.is_empty() {
                bail!(
                    &meta.delimiter.span().join(),
                    "expected at least one parameter in parentheses"
                );
            }
        }
        syn::Meta::Path(path) => bail!(
            &meta,
            "this empty `#[{0}]` attribute is unexpected; \
            remove it or pass some parameters in parentheses: \
            `#[{0}(...)]`",
            darling::util::path_to_string(path)
        ),
        syn::Meta::NameValue(_) => {}
    }

    Ok(())
}
