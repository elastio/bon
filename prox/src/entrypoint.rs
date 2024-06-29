use crate::Result;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

/// Parse a pair of [`proc_macro::TokenStream`] that an attribute macro accepts
/// into a structured input.
fn parse_attr_macro_input<Opts, Item>(opts: TokenStream, item: TokenStream) -> Result<(Opts, Item)>
where
    Opts: darling::FromMeta,
    Item: syn::parse::Parse,
{
    let meta = darling::ast::NestedMeta::parse_meta_list(opts.into())?;
    let opts = Opts::from_list(&meta)?;
    let item = syn::parse(item)?;
    Ok((opts, item))
}

/// Parses the input for a proc macro attribute, runs the macro implementation
/// and automatically converts the [`Result`] into a [`TokenStream`] that contains
/// a nice compilation error.
pub fn proc_macro_attribute<Opts, Item>(
    macro_imp: impl FnOnce(Opts, Item) -> Result<TokenStream2>,
    opts: TokenStream,
    item: TokenStream,
) -> TokenStream
where
    Opts: darling::FromMeta,
    Item: syn::parse::Parse,
{
    parse_attr_macro_input(opts, item)
        .and_then(|(opts, item)| macro_imp(opts, item))
        .unwrap_or_else(darling::Error::write_errors)
        .into()
}

/// Parses the input for a proc macro derive, runs the macro implementation
/// and automatically converts the [`Result`] into a [`TokenStream`] that contains
/// a nice compilation error.
pub fn proc_macro_derive<Input>(
    macro_imp: impl FnOnce(Input) -> Result<TokenStream2>,
    input: TokenStream,
) -> TokenStream
where
    Input: darling::FromDeriveInput,
{
    syn::parse(input)
        .map_err(darling::Error::from)
        .and_then(|input| Input::from_derive_input(&input))
        .and_then(macro_imp)
        .unwrap_or_else(darling::Error::write_errors)
        .into()
}

/// Parses the input for a proc macro (function-like), runs the macro implementation
/// and automatically converts the [`Result`] into a [`TokenStream`] that contains
/// a nice compilation error.
pub fn proc_macro<Input>(
    macro_imp: impl FnOnce(Input) -> Result<TokenStream2>,
    input: TokenStream,
) -> TokenStream
where
    Input: syn::parse::Parse,
{
    syn::parse(input)
        .map_err(darling::Error::from)
        .and_then(macro_imp)
        .unwrap_or_else(darling::Error::write_errors)
        .into()
}
