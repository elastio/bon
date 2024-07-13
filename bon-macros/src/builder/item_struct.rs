use super::builder_gen::input_struct::{StructInputCtx, StructInputParams};
use super::builder_gen::MacroOutput;
use prox::prelude::*;
use quote::quote;
use syn::visit_mut::VisitMut;

pub(crate) fn generate(
    params: StructInputParams,
    orig_struct: syn::ItemStruct,
) -> Result<TokenStream2> {
    let ctx = StructInputCtx::new {
        params,
    };

    let MacroOutput {
        start_func,
        other_items,
    } = ctx.into_builder_gen_ctx()?.output();

    Ok(quote! {
        #start_func
        #other_items
        #orig_struct
    })
}
