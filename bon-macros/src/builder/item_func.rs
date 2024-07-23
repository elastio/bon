use super::builder_gen::input_func::{FuncInputCtx, FuncInputParams};
use super::builder_gen::MacroOutput;
use crate::util::prelude::*;
use quote::quote;
use syn::visit_mut::VisitMut;

pub(crate) fn generate(params: FuncInputParams, orig_func: syn::ItemFn) -> Result<TokenStream2> {
    let mut norm_func = orig_func.clone();

    crate::normalization::NormalizeLifetimes.visit_item_fn_mut(&mut norm_func);
    crate::normalization::NormalizeImplTraits.visit_item_fn_mut(&mut norm_func);

    let ctx = FuncInputCtx {
        orig_func,
        norm_func,
        impl_ctx: None,
        params,
    };

    let adapted_func = ctx.adapted_func()?;

    let MacroOutput {
        start_func,
        other_items,
    } = ctx.into_builder_gen_ctx()?.output()?;

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
