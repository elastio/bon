use easy_ext::ext;

#[ext(FnArgExt)]
pub impl syn::FnArg {
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
}
