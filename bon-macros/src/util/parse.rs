use crate::util::prelude::*;
use std::fmt::Display;

pub(crate) fn visibility(meta: &syn::Meta) -> Result<syn::Visibility> {
    let error = |prefix: &dyn Display, path: &str| {
        err!(
            meta,
            "{prefix}; use the following syntax to \
            specify the visibility instead: `{path}(pub(...))`; if you intended \
            to specify private visibility, then use `{path}(pub(self))`"
        )
    };

    let meta = match meta {
        syn::Meta::NameValue(name_val) => {
            let path = darling::util::path_to_string(&name_val.path);
            return Err(error(
                &format_args!("`{path} = ...` syntax is not supported"),
                &path,
            ));
        }
        syn::Meta::Path(path) => {
            let path = darling::util::path_to_string(path);
            return Err(error(&"missing visibility value", &path));
        }
        syn::Meta::List(meta) => meta,
    };

    meta.require_paren_delim()?;

    if meta.tokens.is_empty() {
        let path = darling::util::path_to_string(&meta.path);
        return Err(error(&"missing visibility value in parentheses", &path));
    }

    let visibility = syn::parse2::<syn::Visibility>(meta.tokens.clone())?;

    Ok(visibility)
}
