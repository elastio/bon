mod builder_gen;

pub(crate) mod item_impl;

mod item_fn;
mod item_struct;

use crate::normalization::{ExpandCfg, ExpansionOutput};
use crate::util;
use crate::util::prelude::*;
use darling::FromMeta;
use quote::quote;
use syn::parse::Parser;

pub(crate) fn generate_from_derive(item: TokenStream) -> TokenStream {
    try_generate_from_derive(item).unwrap_or_else(Error::write_errors)
}

fn try_generate_from_derive(item: TokenStream) -> Result<TokenStream> {
    match syn::parse2(item)? {
        syn::Item::Struct(item_struct) => item_struct::generate(item_struct),
        _ => bail!(
            &Span::call_site(),
            "only `struct` items are supported by the `#[derive(bon::Builder)]` attribute"
        ),
    }
}

pub(crate) fn generate_from_attr(params: TokenStream, item: TokenStream) -> TokenStream {
    try_generate_from_attr(params.clone(), item.clone()).unwrap_or_else(|err| {
        [
            generate_completion_triggers(params),
            crate::error::error_into_token_stream(err, item),
        ]
        .concat()
    })
}

fn try_generate_from_attr(params: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let item: syn::Item = syn::parse2(item)?;

    if let syn::Item::Struct(item_struct) = item {
        return Ok(quote! {
            // Triggers a deprecation warning if the user is using the old attribute
            // syntax on the structs instead of the derive syntax.
            use ::bon::private::deprecations::builder_attribute_on_a_struct as _;

            #[derive(::bon::Builder)]
            #[builder(#params)]
            #item_struct
        });
    }

    let macro_path = syn::parse_quote!(::bon::builder);

    let ctx = ExpandCfg {
        macro_path,
        params,
        item,
    };

    let (params, item) = match ctx.expand_cfg()? {
        ExpansionOutput::Expanded { params, item } => (params, item),
        ExpansionOutput::Recurse(output) => return Ok(output),
    };

    let nested_meta = &darling::ast::NestedMeta::parse_meta_list(params.clone())?;

    let main_output = match item {
        syn::Item::Fn(item_fn) => item_fn::generate(FromMeta::from_list(nested_meta)?, item_fn)?,
        _ => bail!(
            &Span::call_site(),
            "only `fn` items are supported by the `#[bon::builder]` attribute"
        ),
    };

    let output = [generate_completion_triggers(params), main_output].concat();

    Ok(output)
}

fn generate_completion_triggers(params: TokenStream) -> TokenStream {
    let meta = util::ide::parse_comma_separated_meta
        .parse2(params)
        .unwrap_or_default();

    util::ide::generate_completion_triggers(meta)
}
