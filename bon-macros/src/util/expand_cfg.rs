use crate::util::prelude::*;

pub(crate) enum ExpansionOutput {
    Expanded,
    Recurse(TokenStream2),
}

pub(crate) fn expand_cfg(
    _macro_path: syn::Path,
    _params: &mut TokenStream2,
    _item: &mut syn::Item,
) -> ExpansionOutput {
    todo!()
}
