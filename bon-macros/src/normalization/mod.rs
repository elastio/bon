mod impl_traits;
mod lifetimes;

pub(crate) mod self_ty;

use syn::visit_mut::VisitMut;

pub(crate) struct Normalize;

impl VisitMut for Normalize {
    fn visit_item_impl_mut(&mut self, impl_block: &mut syn::ItemImpl) {
        lifetimes::NormalizeLifetimes.visit_item_impl_mut(impl_block);
        impl_traits::NormalizeImplTraits.visit_item_impl_mut(impl_block);

        self_ty::NormalizeSelfTy {
            self_ty: &impl_block.self_ty.clone(),
        }
        .visit_item_impl_mut(impl_block);
    }

    fn visit_item_fn_mut(&mut self, item_fn: &mut syn::ItemFn) {
        lifetimes::NormalizeLifetimes.visit_item_fn_mut(item_fn);
        impl_traits::NormalizeImplTraits.visit_item_fn_mut(item_fn);
    }
}

fn as_builder_impl_item_fn(impl_item: &mut syn::ImplItem) -> Option<&mut syn::ImplItemFn> {
    let syn::ImplItem::Fn(fn_item) = impl_item else {
        return None;
    };

    let is_builder_fn = fn_item
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("builder"));

    is_builder_fn.then_some(fn_item)
}
