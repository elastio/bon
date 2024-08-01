mod attrs;
mod fn_arg;
mod ident;
mod path;
mod ty;

use prelude::*;
use proc_macro::TokenStream;
use syn::parse::ParseStream;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::Token;

pub(crate) mod prelude {
    /// A handly alias for [`proc_macro2::TokenStream`].
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

pub(crate) fn parse_map_macro_input(
    input: ParseStream<'_>,
) -> Result<Punctuated<(Expr, Expr), Token![,]>, syn::Error> {
    Punctuated::parse_terminated_with(input, parse_map_pair)
}

fn parse_map_pair(pair: ParseStream<'_>) -> Result<(Expr, Expr), syn::Error> {
    let key = pair.parse().map_err(|_| pair.error(pair.to_string()))?;
    let _: Token![:] = pair.parse()?;
    let value = pair.parse()?;

    Ok((key, value))
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
