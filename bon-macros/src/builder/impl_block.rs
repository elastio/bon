use super::{ImplBlock, MacroCtx, MacroOutput};
use itertools::{Either, Itertools};
use prox::prelude::*;
use quote::quote;
use syn::visit_mut::VisitMut;

pub(crate) fn generate_for_impl_block(mut impl_block: syn::ItemImpl) -> Result<TokenStream2> {
    if let Some((_, trait_path, _)) = &impl_block.trait_ {
        bail!(trait_path, "Impls of traits are not supported yet");
    }

    crate::normalization::Normalize.visit_item_impl_mut(&mut impl_block);

    let (other_items, builder_funcs): (Vec<_>, Vec<_>) =
        impl_block.items.into_iter().partition_map(|item| {
            let syn::ImplItem::Fn(mut fn_item) = item else {
                return Either::Left(item);
            };

            let builder_attr_index = fn_item
                .attrs
                .iter()
                .position(|attr| attr.path().is_ident("builder"));

            let Some(builder_attr_index) = builder_attr_index else {
                return Either::Left(syn::ImplItem::Fn(fn_item));
            };

            fn_item.attrs.remove(builder_attr_index);

            Either::Right(fn_item)
        });

    if builder_funcs.is_empty() {
        bail!(
            &proc_macro2::Span::call_site(),
            "There are no #[builder] functions in the impl block, so there is no \
            need for a #[bon] attribute on the impl block"
        );
    }

    let outputs: Vec<_> = builder_funcs
        .into_iter()
        .map(|fn_item| {
            let impl_item = ImplBlock {
                self_ty: &impl_block.self_ty,
                generics: &impl_block.generics,
            };

            generate_for_assoc_fn_item(impl_item, fn_item)
        })
        .try_collect()?;

    let new_impl_items = outputs.iter().flat_map(|output| {
        let entry_func = &output.entry_func;
        let positional_func = &output.positional_func;
        [
            syn::parse_quote!(#entry_func),
            syn::parse_quote!(#positional_func),
        ]
    });

    impl_block.items = other_items;
    impl_block.items.extend(new_impl_items);

    let other_items = outputs.iter().map(|output| &output.other_items);

    Ok(quote! {
        #impl_block
        #(#other_items)*
    })
}

fn generate_for_assoc_fn_item(
    impl_item: ImplBlock<'_>,
    func: syn::ImplItemFn,
) -> Result<MacroOutput> {
    let syn::ImplItemFn {
        attrs,
        vis,
        defaultness,
        sig,
        block,
    } = func;

    if let Some(defaultness) = &defaultness {
        bail!(defaultness, "Default functions are not supported yet");
    }

    let func = syn::ItemFn {
        attrs,
        vis,
        sig,
        block: Box::new(block),
    };

    let ctx = MacroCtx::new(func, Some(impl_item))?;

    Ok(ctx.output())
}
