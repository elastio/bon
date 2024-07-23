use easy_ext::ext;

#[ext(FnArgExt)]
pub(crate) impl syn::FnArg {
    fn attrs_mut(&mut self) -> &mut Vec<syn::Attribute> {
        match self {
            syn::FnArg::Receiver(arg) => &mut arg.attrs,
            syn::FnArg::Typed(arg) => &mut arg.attrs,
        }
    }

    fn ty_mut(&mut self) -> &mut syn::Type {
        match self {
            syn::FnArg::Receiver(arg) => &mut arg.ty,
            syn::FnArg::Typed(arg) => &mut arg.ty,
        }
    }

    fn as_receiver(&self) -> Option<&syn::Receiver> {
        match self {
            syn::FnArg::Typed(_) => None,
            syn::FnArg::Receiver(arg) => Some(arg),
        }
    }

    fn as_typed(&self) -> Option<&syn::PatType> {
        match self {
            syn::FnArg::Typed(arg) => Some(arg),
            syn::FnArg::Receiver(_) => None,
        }
    }
}
