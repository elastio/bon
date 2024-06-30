mod builder;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn builder(opts: TokenStream, item: TokenStream) -> TokenStream {
    prox::parse_attr_macro_input(opts, item.clone())
        .and_then(|(opts, item)| builder::generate(opts, item))
        .unwrap_or_else(|err| builder::error_into_token_stream(err, item.into()))
        .into()
}
