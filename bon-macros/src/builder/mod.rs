mod builder_gen;

pub(crate) mod item_impl;

mod item_fn;
mod item_struct;

use crate::normalization::{ExpandCfg, ExpansionOutput, GenericsNamespace};
use crate::util;
use crate::util::prelude::*;
use darling::FromMeta;
use syn::parse::Parser;
use syn::visit::Visit;

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
        syn::Item::Fn(item_fn) => {
            let mut namespace = GenericsNamespace::default();
            namespace.visit_item_fn(&item_fn);

            item_fn::generate(FromMeta::from_list(nested_meta)?, item_fn, &namespace)?
        }
        syn::Item::Struct(struct_item) => {
            bail!(
                &struct_item.struct_token,
                "to generate a builder for a struct, use `#[derive(bon::Builder)]` instead; \
                 `#[bon::builder]` syntax is supported only for functions"
            )
        }
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
