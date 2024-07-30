use crate::{builder::builder_gen::input_enum::NamedVariantInputCtx, util::prelude::*};

pub(crate) fn generate(
    orig_enum : syn::ItemEnum,
) -> Result<TokenStream2> {
    let mut token_stream = TokenStream2::new();

    for variant in orig_enum.variants.clone() {
        match variant.fields.clone() {
            // Skip Unit??
            syn::Fields::Unit => (),
            syn::Fields::Unnamed(fields) => todo!(),
            syn::Fields::Named(fields) => {
                let ctx = NamedVariantInputCtx::new(&orig_enum, &variant, fields);
                token_stream.extend(ctx.generate()?);
            }
        }
    }

    Ok(token_stream)
}
