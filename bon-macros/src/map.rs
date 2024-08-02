use crate::util;
use crate::util::prelude::*;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::Token;

pub(crate) fn generate(entries: Punctuated<(Expr, Expr), Token![,]>) -> Result<TokenStream2> {
    util::ensure_unique(entries.iter().map(|(k, _)| k))?;

    let items = entries.into_iter().map(|(key, value)| {
        let key = quote!(::core::convert::Into::into(#key));
        let value = quote!(::core::convert::Into::into(#value));
        quote!((#key, #value))
    });

    Ok(quote! {
        ::core::iter::FromIterator::from_iter([
            #(#items),*
        ])
    })
}
