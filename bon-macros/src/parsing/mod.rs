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
use syn::parse::Parser;
use syn::punctuated::Punctuated;

pub(crate) fn parse_non_empty_paren_meta_list<T: FromMeta>(meta: &syn::Meta) -> Result<T> {
    require_non_empty_paren_meta_list_or_name_value(meta)?;
    T::from_meta(meta)
}

pub(crate) fn require_non_empty_paren_meta_list_or_name_value(meta: &syn::Meta) -> Result {
    match meta {
        syn::Meta::List(meta) => {
            meta.require_parens_delim()?;

            if meta.tokens.is_empty() {
                bail!(
                    &meta.delimiter.span().join(),
                    "expected parameters in parentheses"
                );
            }
        }
        syn::Meta::Path(path) => bail!(
            &meta,
            "this empty `#[{0}]` attribute is unexpected; \
            remove it or pass parameters in parentheses: \
            `#[{0}(...)]`",
            darling::util::path_to_string(path)
        ),
        syn::Meta::NameValue(_) => {}
    }

    Ok(())
}

/// Utility for parsing with `#[darling(with = ...)]` attribute that allows to
/// parse an arbitrary sequence of items inside of parentheses. For example
/// `foo(a, b, c)`, where `a`, `b`, and `c` are of type `T` and `,` is represented
/// by the token type `P`.
#[allow(dead_code)]
pub(crate) fn parse_paren_meta_list_with_terminated<T, P>(
    meta: &syn::Meta,
) -> Result<Punctuated<T, P>>
where
    T: syn::parse::Parse,
    P: syn::parse::Parse,
{
    let item = std::any::type_name::<T>();
    let punct = std::any::type_name::<P>();

    let name = |val: &str| {
        format!(
            "'{}'",
            val.rsplit("::").next().unwrap_or(val).to_lowercase()
        )
    };

    let meta = match meta {
        syn::Meta::List(meta) => meta,
        _ => bail!(
            &meta,
            "expected a list of {} separated by {}",
            name(item),
            name(punct),
        ),
    };

    meta.require_parens_delim()?;

    let punctuated = Punctuated::parse_terminated.parse2(meta.tokens.clone())?;

    Ok(punctuated)
}
