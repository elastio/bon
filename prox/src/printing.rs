/// Uses [`prettyplease`] to format [`syn::Path`] to [`String`].
pub fn path_to_string(path: &syn::Path) -> String {
    prettyplease::unparse(&syn::parse_quote!(use #path;))
        .strip_prefix("use ")
        .unwrap()
        .strip_suffix(";\n")
        .unwrap()
        .to_owned()
}
