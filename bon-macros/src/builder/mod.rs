mod builder_gen;
mod params;

pub(crate) mod item_impl;

mod item_func;
mod item_struct;

use crate::util;
use crate::util::prelude::*;
use crate::normalization::{ExpansionOutput, expand_cfg};
use darling::FromMeta;
use syn::parse::Parser;

fn generate_for_item(params: TokenStream2, item: syn::Item) -> Result<TokenStream2> {
    let params = &darling::ast::NestedMeta::parse_meta_list(params)?;

    match item {
        syn::Item::Fn(item) => item_func::generate(FromMeta::from_list(params)?, item),
        syn::Item::Struct(item) => item_struct::generate(FromMeta::from_list(params)?, item),
        _ => {
            bail!(
                &item,
                "The attribute is expected to be placed only on an `fn` \
                item or a `struct` declaration"
            )
        }
    }
}

pub(crate) fn generate(params: TokenStream2, item: TokenStream2) -> TokenStream2 {
    try_generate(params.clone(), item.clone()).unwrap_or_else(|err| {
        [
            generate_completion_triggers(params),
            crate::error::error_into_token_stream(err, item),
        ]
        .concat()
    })
}

fn try_generate(mut params: TokenStream2, item: TokenStream2) -> Result<TokenStream2> {
    let mut item: syn::Item = syn::parse2(item)?;
    let macro_path = syn::parse_quote!(::bon::builder);

    match expand_cfg(macro_path, &mut params, &mut item)? {
        ExpansionOutput::Expanded => {}
        ExpansionOutput::Recurse(output) => return Ok(output),
    }

    let output = [
        generate_completion_triggers(params.clone()),
        generate_for_item(params, item)?,
    ]
    .concat();

    Ok(output)
}

fn generate_completion_triggers(params: TokenStream2) -> TokenStream2 {
    let meta = util::ide::parse_comma_separated_meta
        .parse2(params)
        .unwrap_or_default();

    util::ide::generate_completion_triggers(meta)
}
