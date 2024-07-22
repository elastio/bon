!#[doc = include_str!("../README.md")]

mod bon;
mod builder;
mod error;
mod normalization;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn builder(params: TokenStream, item: TokenStream) -> TokenStream {
    syn::parse(item.clone())
        .map_err(Into::into)
        .and_then(|item| builder::generate_for_item(params.into(), item))
        .unwrap_or_else(|err| error::error_into_token_stream(err, item.into()))
        .into()
}

#[proc_macro_attribute]
pub fn bon(params: TokenStream, item: TokenStream) -> TokenStream {
    prox::parse_attr_macro_input(params, item.clone())
        .and_then(|(opts, item)| bon::generate(opts, item))
        .unwrap_or_else(|err| error::error_into_token_stream(err, item.into()))
        .into()
}
