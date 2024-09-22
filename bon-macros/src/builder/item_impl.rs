use super::builder_gen::input_fn::{FnInputCtx, FnInputParams, ImplCtx};
use crate::util::prelude::*;
use darling::ast::NestedMeta;
use darling::FromMeta;
use quote::quote;
use std::rc::Rc;
use syn::visit_mut::VisitMut;

pub(crate) fn generate(mut orig_impl_block: syn::ItemImpl) -> Result<TokenStream2> {
    if let Some((_, trait_path, _)) = &orig_impl_block.trait_ {
        bail!(trait_path, "Impls of traits are not supported yet");
    }

    let (builder_fns, other_items): (Vec<_>, Vec<_>) =
        orig_impl_block.items.into_iter().partition(|item| {
            let fn_item = match item {
                syn::ImplItem::Fn(fn_item) => fn_item,
                _ => return false,
            };

            fn_item
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("builder"))
        });

    if builder_fns.is_empty() {
        bail!(
            &Span::call_site(),
            "There are no #[builder] functions in the impl block, so there is no \
            need for a #[bon] attribute on the impl block"
        );
    }

    orig_impl_block.items = builder_fns;

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

    // Retain a variant of the impl block without the normalized `Self` mentions.
    // This way we preserve the original code that the user wrote with `Self` mentions
    // as much as possible, therefore IDE's are able to provide correct syntax highlighting
    // for `Self` mentions, because they aren't removed from the generated code output
    let mut norm_selfful_impl_block = norm_impl_block.clone();

    crate::normalization::NormalizeSelfTy {
        self_ty: &norm_impl_block.self_ty.clone(),
    }
    .visit_item_impl_mut(&mut norm_impl_block);

    let impl_ctx = Rc::new(ImplCtx {
        self_ty: norm_impl_block.self_ty,
        generics: norm_impl_block.generics,
        allow_attrs: norm_impl_block
            .attrs
            .iter()
            .filter_map(syn::Attribute::to_allow)
            .collect(),
    });

    let outputs = orig_impl_block
        .items
        .into_iter()
        .zip(norm_impl_block.items)
        .map(|(orig_item, norm_item)| {
            let norm_fn = match norm_item {
                syn::ImplItem::Fn(norm_fn) => norm_fn,
                _ => unreachable!(),
            };
            let orig_fn = match orig_item {
                syn::ImplItem::Fn(orig_fn) => orig_fn,
                _ => unreachable!(),
            };

            let norm_fn = impl_item_fn_into_fn_item(norm_fn)?;
            let orig_fn = impl_item_fn_into_fn_item(orig_fn)?;

            let meta = orig_fn
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("builder"))
                .map(|attr| {
                    let meta_list = darling::util::parse_attribute_to_meta_list(attr)?;
                    NestedMeta::parse_meta_list(meta_list.tokens).map_err(Into::into)
                })
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            let params = FnInputParams::from_list(&meta)?;

            let ctx = FnInputCtx {
                orig_fn,
                norm_fn,
                impl_ctx: Some(impl_ctx.clone()),
                params,
            };

            Result::<_>::Ok((ctx.adapted_fn()?, ctx.into_builder_gen_ctx()?.output()?))
        })
        .collect::<Result<Vec<_>>>()?;

    let new_impl_items = outputs.iter().flat_map(|(adapted_fn, output)| {
        let start_fn = &output.start_fn;
        [syn::parse_quote!(#start_fn), syn::parse_quote!(#adapted_fn)]
    });

    norm_selfful_impl_block.items = other_items;
    norm_selfful_impl_block.items.extend(new_impl_items);

    let other_items = outputs.iter().map(|(_, output)| &output.other_items);

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
