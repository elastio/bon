use crate::util;
use crate::util::prelude::*;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::Token;

pub(crate) fn generate(entries: Punctuated<Expr, Token![,]>) -> Result<TokenStream2> {
    util::ensure_unique(entries.iter())?;

    let entries = entries.into_iter();

    Ok(quote! {
        ::core::iter::FromIterator::from_iter([
            #(#items),*
        ])
    })
}
