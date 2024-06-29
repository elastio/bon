mod builder;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn builder(opts: TokenStream, item: TokenStream) -> TokenStream {
    prox::proc_macro_attribute(builder::generate, opts, item)
}
