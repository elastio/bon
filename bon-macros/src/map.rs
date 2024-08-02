use std::collections::HashSet;

use crate::util::prelude::*;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::Token;

pub(crate) fn generate(entries: Punctuated<(Expr, Expr), Token![,]>) -> Result<TokenStream2> {
    ensure_unique_keys(entries.iter().map(|(k, _)| k))?;

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

fn ensure_unique_keys<'k, I>(keys: I) -> Result<()>
where
    I: IntoIterator<Item = &'k Expr>,
{
    let mut errors = Error::accumulator();

    let mut exprs = HashSet::new();

    keys.into_iter().for_each(|key| {
        if !exprs.insert(key.clone()) {
            errors.push(err!(key, "duplicate map key"));
        }
    });

    errors.finish()?;

    Ok(())
}
