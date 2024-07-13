//! This is a proc-macro crate that is supposed to be a private implementation detail
//! of the [`bon`] crate. Don't use it directly! The API here is
//! unstable, and your code may break if you do. Instead use the proc macros from here
//! via the reexports in the [`bon`] crate.
//!
//! [`bon``]: https://docs.rs/bon

mod bon;
mod builder;
mod error;
mod normalization;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn builder(opts: TokenStream, item: TokenStream) -> TokenStream {
    prox::parse_attr_macro_input(opts, item.clone())
        .and_then(|(opts, item)| builder::generate(opts, item))
        .unwrap_or_else(|err| error::error_into_token_stream(err, item.into()))
        .into()
}

#[proc_macro_attribute]
pub fn bon(opts: TokenStream, item: TokenStream) -> TokenStream {
    prox::parse_attr_macro_input(opts, item.clone())
        .and_then(|(opts, item)| bon::generate(opts, item))
        .unwrap_or_else(|err| error::error_into_token_stream(err, item.into()))
        .into()
}
