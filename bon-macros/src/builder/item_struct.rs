use super::builder_gen::input_struct::{StructInputCtx, StructInputParams};
use super::builder_gen::MacroOutput;
use crate::util::prelude::*;
use quote::quote;

pub(crate) fn generate(
    params: StructInputParams,
    orig_struct: syn::ItemStruct,
) -> Result<TokenStream2> {
    let ctx = StructInputCtx::new(params, orig_struct);

    let adapted_struct = ctx.adapted_struct();

    let MacroOutput {
        mut start_func,
        other_items,
    } = ctx.into_builder_gen_ctx()?.output()?;

    let impl_generics = std::mem::take(&mut start_func.sig.generics);

    let (generics_decl, generic_args, where_clause) = impl_generics.split_for_impl();

    let struct_ident = &adapted_struct.ident;

    Ok(quote! {
        #[automatically_derived]
        impl #generics_decl #struct_ident #generic_args
            #where_clause
        {
            #start_func
        }

        #other_items
        #adapted_struct
    })
}
