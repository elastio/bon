mod builder_gen;
mod params;

pub(crate) mod item_impl;

mod item_func;
mod item_struct;

use crate::util::prelude::*;
use darling::FromMeta;

pub(crate) fn generate_for_item(params: TokenStream2, item: syn::Item) -> Result<TokenStream2> {
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
