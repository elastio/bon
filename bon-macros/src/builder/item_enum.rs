use quote::quote;

use crate::util::prelude::*;

pub(crate) fn generate(
    orig_enum : syn::ItemEnum,
) -> Result<TokenStream2> {

    let mut token_stream = TokenStream2::new();
    for variant in orig_enum.variants {
        match variant.fields {
            // Skip Unit??
            syn::Fields::Unit => (),
            syn::Fields::Unnamed(fields) => todo!()
            syn::Fields::Named(_) => todo!(),
        }
    }

    todo!()
}
