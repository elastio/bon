use super::builder_gen::input_fn::{FnInputCtx, FnInputParams};
use super::builder_gen::MacroOutput;
use crate::normalization::SyntaxVariant;
use crate::util::prelude::*;
use syn::visit_mut::VisitMut;

pub(crate) fn generate(params: FnInputParams, orig_fn: syn::ItemFn) -> Result<TokenStream> {
    let mut norm_fn = orig_fn.clone();

    crate::normalization::NormalizeLifetimes.visit_item_fn_mut(&mut norm_fn);
    crate::normalization::NormalizeImplTraits.visit_item_fn_mut(&mut norm_fn);

    let fn_item = SyntaxVariant {
        orig: orig_fn,
        norm: norm_fn,
    };

    let ctx = FnInputCtx {
        fn_item,
        impl_ctx: None,
        params,
    };

    let adapted_fn = ctx.adapted_fn()?;

    let MacroOutput {
        start_fn,
        other_items,
    } = ctx.into_builder_gen_ctx()?.output()?;

    Ok(quote! {
        #start_fn
        #other_items

        // Keep original function at the end. It seems like rust-analyzer
        // does better job of highlighting syntax when it is here. Assuming
        // this is because rust-analyzer prefers the last occurrence of the
        // span when highlighting.
        #adapted_fn
    })
}
