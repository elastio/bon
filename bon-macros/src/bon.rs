use crate::builder;
use darling::FromMeta;
use crate::util::prelude::*;

#[derive(Debug, FromMeta)]
pub(crate) struct BonParams {
    // There may be config options for the proc macro in the future here
}

pub(crate) fn generate(_: BonParams, item: syn::Item) -> Result<TokenStream2> {
    match item {
        syn::Item::Impl(item_impl) => builder::item_impl::generate(item_impl),
        _ => bail!(
            &item,
            "`#[bon]` attribute is expected to be placed on an `impl` block \
             but it was placed on other syntax instead"
        ),
    }
}
