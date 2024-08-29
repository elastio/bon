mod attrs;
mod fn_arg;
mod ident;
mod iterator;
mod path;
mod ty;

pub(crate) mod ide;

use prelude::*;
use proc_macro::TokenStream;
use std::collections::HashSet;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::Expr;

pub(crate) mod prelude {
    /// A handy alias for [`proc_macro2::TokenStream`].
    pub(crate) use proc_macro2::TokenStream as TokenStream2;

    /// The `Error` type in in this crate is supposed to act like `anyhow::Error`
    /// providing a simple way to create and return errors from format strings.
    ///
    /// See [`err!()`] and [`bail!()`] macros for creating errors. Right now this
    /// is just a reexport of [`darling::Error`] because that error already provides
    /// the anyhow-like error handling experience.
    pub(crate) use darling::Error;

    pub(crate) type Result<T = (), E = Error> = std::result::Result<T, E>;

    pub(crate) use super::attrs::AttributeExt;
    pub(crate) use super::fn_arg::FnArgExt;
    pub(crate) use super::ident::IdentExt;
    pub(crate) use super::iterator::IntoIteratorExt;
    pub(crate) use super::iterator::IteratorExt;
    pub(crate) use super::path::PathExt;
    pub(crate) use super::ty::TypeExt;
    pub(crate) use super::{bail, err};
}

/// Parse a pair of [`proc_macro::TokenStream`] that an attribute macro accepts
/// into a structured input.
pub(crate) fn parse_attr_macro_input<Params, Item>(
    params: TokenStream,
    item: TokenStream,
) -> Result<(Params, Item)>
where
    Params: darling::FromMeta,
    Item: syn::parse::Parse,
{
    let meta = darling::ast::NestedMeta::parse_meta_list(params.into())?;
    let params = Params::from_list(&meta)?;
    let item = syn::parse(item)?;
    Ok((params, item))
}

/// Utility for parsing with `#[darling(with = ...)]` attribute that allows to
/// parse an arbitrary sequence of items inside of parentheses. For example
/// `foo(a, b, c)`, where `a`, `b`, and `c` are of type `T` and `,` is represented
/// by the token type `P`.
// This function was used by earlier iterations of our code, but it's not used now
// However, let's keep it here for future.
#[allow(dead_code)]
pub(crate) fn parse_terminated<T, P>(meta: &syn::Meta) -> Result<Punctuated<T, P>>
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

    let tokens = match meta {
        syn::Meta::List(meta) => &meta.tokens,
        _ => bail!(
            &meta,
            "expected a list of {} separated by {}",
            name(item),
            name(punct),
        ),
    };

    let punctuated = Punctuated::parse_terminated.parse2(tokens.clone())?;

    Ok(punctuated)
}

pub(crate) fn validate_expressions_are_unique<'k, I>(err_label: &str, items: I) -> Option<Error>
where
    I: IntoIterator<Item = &'k Expr>,
{
    let mut errors = Error::accumulator();

    let mut exprs = HashSet::new();

    items
        .into_iter()
        .filter(|item| is_pure(item))
        .for_each(|new_item| {
            let Some(existing) = exprs.replace(new_item) else {
                return;
            };
            errors.extend([
                err!(existing, "duplicate {err_label}"),
                err!(new_item, "duplicate {err_label}"),
            ]);
        });

    errors.finish().err()
}

fn is_pure(expr: &Expr) -> bool {
    match expr {
        Expr::Binary(binary) => is_pure(&binary.left) && is_pure(&binary.right),
        Expr::Group(group) => is_pure(&group.expr),
        Expr::Lit(_) => true,
        Expr::Paren(paren) => is_pure(&paren.expr),
        Expr::Unary(unary) => is_pure(&unary.expr),
        _ => false,
    }
}

/// Inspired by `anyhow::bail`, but returns a [`Result`] with [`darling::Error`].
/// It accepts the value that implements [`syn::spanned::Spanned`] to attach the
/// span to the error.
macro_rules! bail {
    ($spanned:expr, $($tt:tt)*) => {
        return Err($crate::util::err!($spanned, $($tt)*))
    };
}

/// Inspired by `anyhow::anyhow`, but returns a [`darling::Error`].
/// It accepts the value that implements [`syn::spanned::Spanned`] to attach the
/// span to the error.
macro_rules! err {
    ($spanned:expr, $($tt:tt)*) => {
        ::darling::Error::custom(format_args!($($tt)*)).with_span($spanned)
    };
}

pub(crate) use {bail, err};
