use super::builder_gen::MacroOutput;
use super::func_input::{FuncInputCtx, FuncInputParams};
use prox::prelude::*;
use quote::quote;
use syn::visit_mut::VisitMut;

pub(crate) fn generate(params: FuncInputParams, item: syn::Item) -> Result<TokenStream2> {
    let orig_func = match item {
        syn::Item::Fn(orig_func) => orig_func,
        _ => prox::bail!(
            &item,
            "The attribute is expected to be placed on an `fn` \
            item, but it was placed on other syntax instead"
        ),
    };

    let mut norm_func = orig_func.clone();

    crate::normalization::NormalizeLifetimes.visit_item_fn_mut(&mut norm_func);
    crate::normalization::NormalizeImplTraits.visit_item_fn_mut(&mut norm_func);

    let ctx = FuncInputCtx {
        orig_func,
        norm_func,
        impl_ctx: None,
        params,
    };

    let adapted_func = ctx.adapted_func();

    let MacroOutput {
        start_func,
        other_items,
    } = ctx.into_builder_gen_ctx()?.output();

    Ok(quote! {
        #start_func
        #other_items

        // Keep original function at the end. It seems like rust-analyzer
        // does better job of highlighting syntax when it is here. Assuming
        // this is because rust-analyzer prefers the last occurrence of the
        // span when highlighting.
        #adapted_func
    })
}
