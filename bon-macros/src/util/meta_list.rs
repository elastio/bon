use crate::util::prelude::*;

pub(crate) trait MetaListExt {
    fn require_paren_delim(&self) -> Result<()>;
}

impl MetaListExt for syn::MetaList {
    fn require_paren_delim(&self) -> Result<()> {
        if matches!(self.delimiter, syn::MacroDelimiter::Paren(_)) {
            return Ok(());
        }

        let path = darling::util::path_to_string(&self.path);

        bail!(
            self,
            "wrong delimiter, expected parentheses e.g. `{path}(...)`, but got {}",
            delim_example(&path, &self.delimiter),
        );
    }
}

fn delim_example(path: &str, delimiter: &syn::MacroDelimiter) -> String {
    match delimiter {
        syn::MacroDelimiter::Paren(_) => format!("`{path}(...)`"),
        syn::MacroDelimiter::Brace(_) => format!("`{path}{{...}}`"),
        syn::MacroDelimiter::Bracket(_) => format!("`{path}[...]`"),
    }
}
