use super::{ImplCtx, MacroCtx};
use itertools::{Either, Itertools};
use prox::prelude::*;
use quote::quote;
use syn::visit_mut::VisitMut;

pub(crate) fn generate_for_impl_block(mut orig_impl_block: syn::ItemImpl) -> Result<TokenStream2> {
    if let Some((_, trait_path, _)) = &orig_impl_block.trait_ {
        bail!(trait_path, "Impls of traits are not supported yet");
    }

    let (other_items, builder_funcs): (Vec<_>, Vec<_>) =
        orig_impl_block.items.into_iter().partition_map(|item| {
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

            Either::Right(syn::ImplItem::Fn(fn_item))
        });

    if builder_funcs.is_empty() {
        bail!(
            &proc_macro2::Span::call_site(),
            "There are no #[builder] functions in the impl block, so there is no \
            need for a #[bon] attribute on the impl block"
        );
    }

    orig_impl_block.items = builder_funcs;

    // We do this back-and-forth with normalizing various syntax and saving original
    // to provide cleaner code generation that is easier to consume for IDEs and for
    // rust-analyzer specifically.
    //
    // For codegen logic we would like to have everything normalized. For example, we
    // want to assume `Self` is replaced with the original type and all lifetimes are
    // named, and `impl Traits` are desugared into type parameters.
    //
    // However, in output code we want to preserve existing `Self` references to make
    // sure rust-analyzer highlights them properly. If we just strip `Self` from output
    // code, then rust-analyzer won't be able to associate what `Self` token maps to in
    // the input. It would highlight `Self` as an "unresolved symbol"
    let mut norm_impl_block = orig_impl_block.clone();

    crate::normalization::NormalizeLifetimes.visit_item_impl_mut(&mut norm_impl_block);
    crate::normalization::NormalizeImplTraits.visit_item_impl_mut(&mut norm_impl_block);

    let mut norm_selfful_impl_block = norm_impl_block.clone();

    crate::normalization::NormalizeSelfTy {
        self_ty: &orig_impl_block.self_ty,
    }
    .visit_item_impl_mut(&mut norm_impl_block);

    let outputs: Vec<_> = std::iter::zip(orig_impl_block.items, norm_impl_block.items)
        .map(|(orig_item, norm_item)| {
            let syn::ImplItem::Fn(norm_func) = norm_item else {
                unreachable!();
            };
            let syn::ImplItem::Fn(orig_func) = orig_item else {
                unreachable!();
            };

            let norm_func = impl_item_fn_into_fn_item(norm_func)?;
            let orig_func = impl_item_fn_into_fn_item(orig_func)?;

            let impl_ctx = ImplCtx {
                self_ty: &norm_impl_block.self_ty,
                generics: &norm_impl_block.generics,
            };

            let ctx = MacroCtx::new(orig_func, norm_func, Some(impl_ctx))?;

            Result::<_>::Ok(ctx.output())
        })
        .try_collect()?;

    let new_impl_items = outputs.iter().flat_map(|output| {
        let entry_func = &output.entry_func;
        let adapted_func = &output.adapted_func;
        [
            syn::parse_quote!(#entry_func),
            syn::parse_quote!(#adapted_func),
        ]
    });

    norm_selfful_impl_block.items = other_items;
    norm_selfful_impl_block.items.extend(new_impl_items);

    let other_items = outputs.iter().map(|output| &output.other_items);

    Ok(quote! {
        #(#other_items)*
        #norm_selfful_impl_block
    })
}

fn impl_item_fn_into_fn_item(func: syn::ImplItemFn) -> Result<syn::ItemFn> {
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

    Ok(syn::ItemFn {
        attrs,
        vis,
        sig,
        block: Box::new(block),
    })
}
