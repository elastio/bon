use crate::builder;
use darling::FromMeta;
use prox::prelude::*;

#[derive(Debug, FromMeta)]
pub(crate) struct BonParams {
    // There may be config options for the proc macro in the future here
}

pub(crate) fn generate(_: BonParams, item: syn::Item) -> Result<TokenStream2> {
    match item {
        syn::Item::Impl(impl_item) => builder::generate_for_impl_block(impl_item),
        _ => bail!(
            &item,
            "The attribute is expected to be placed on an `fn` \
            item, but it was placed on other syntax instead"
        ),
    }
}
