use crate::builder;
use crate::normalization::{ExpandCfg, ExpansionOutput};
use crate::util::prelude::*;

pub(crate) fn generate(params: TokenStream, item: TokenStream) -> TokenStream {
    try_generate(params, item.clone())
        .unwrap_or_else(|err| crate::error::error_into_token_stream(err, item))
}

pub(crate) fn try_generate(params: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let item: syn::Item = syn::parse2(item)?;
    let macro_path = syn::parse_quote!(::bon::bon);

    let ctx = ExpandCfg {
        macro_path,
        params,
        item,
    };

    let (params, item) = match ctx.expand_cfg()? {
        ExpansionOutput::Expanded { params, item } => (params, item),
        ExpansionOutput::Recurse(output) => return Ok(output),
    };

    if !params.is_empty() {
        bail!(
            &params,
            "`#[bon]` attribute does not accept any parameters yet, \
            but it will in future releases"
        );
    }

    match item {
        syn::Item::Impl(item_impl) => builder::item_impl::generate(item_impl),
        _ => bail!(
            &item,
            "`#[bon]` attribute is expected to be placed on an `impl` block \
             but it was placed on other syntax instead"
        ),
    }
}
