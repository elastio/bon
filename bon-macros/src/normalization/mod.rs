mod impl_traits;
mod lifetimes;
mod self_ty;

pub(crate) use impl_traits::NormalizeImplTraits;
pub(crate) use lifetimes::NormalizeLifetimes;
pub(crate) use self_ty::NormalizeSelfTy;


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
