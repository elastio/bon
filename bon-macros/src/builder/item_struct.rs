use super::builder_gen::input_struct::StructInputParams;
use super::shared;
use crate::{util::prelude::*,builder::builder_gen::input_struct::StructLiteralBody};

pub(crate) fn generate(
    params: StructInputParams,
    orig_struct: syn::ItemStruct,
) -> Result<TokenStream2> {
    shared::generate(
        params,
        orig_struct,
        // Once https://github.com/rust-lang/rust/issues/35121 is stablized 
        // one can change StructLiteralBody visibility to `struct StructLiteralBody` and remove specific generic paramter`
        |ctx| ctx.into_builder_gen_ctx::<StructLiteralBody>(None)
    )
}
