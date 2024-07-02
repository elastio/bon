use super::{MacroCtx, MacroOutput};
use darling::FromMeta;
use prox::prelude::*;
use quote::quote;
use syn::visit_mut::VisitMut;

#[derive(Debug, FromMeta)]
pub(crate) struct FreeFnItemParams {
    // There may be config options for the proc macro in the future here
}

pub(crate) fn generate_for_free_fn_item(
    _: FreeFnItemParams,
    item: syn::Item,
) -> Result<TokenStream2> {
    match item {
        syn::Item::Fn(mut func) => {
            crate::normalization::Normalize.visit_item_fn_mut(&mut func);

            let ctx = MacroCtx::new(func, None)?;
            let MacroOutput {
                entry_func,
                positional_func,
                other_items,
            } = ctx.output();

            Ok(quote! {
                #entry_func
                #other_items

                // Keep positional function at the end. It seems like rust-analyzer
                // does better job of highlighting syntax when it is here. Assuming
                // this is because rust-analyzer prefers the last occurrence of the
                // span when highlighting.
                #positional_func
            })
        }
        _ => prox::bail!(
            &item,
            "The attribute is expected to be placed on an `fn` \
            item, but it was placed on other syntax instead"
        ),
    }
}
