use crate::util::prelude::*;
use darling::FromMeta;
use std::fmt;
use std::ops::Deref;

/// A type that stores the attribute key path information along with the parsed value.
/// It is useful for error reporting. For example, if some key was unexpected, it's
/// possible to point to the key's span in the error instead of the attribute's value.
pub(crate) struct SpannedKey<T> {
    pub(crate) key: syn::Path,
    pub(crate) value: T,
}

impl<T> SpannedKey<T> {
    pub(crate) fn new(path: &syn::Path, value: T) -> Self {
        Self {
            key: path.clone(),
            value,
        }
    }
    pub(crate) fn key_to_string(&self) -> String {
        darling::util::path_to_string(&self.key)
    }
}

impl<T: FromMeta> FromMeta for SpannedKey<T> {
    fn from_meta(meta: &syn::Meta) -> Result<Self> {
        let key = meta.path().clone();
        let value = T::from_meta(meta)?;
        Ok(Self { key, value })
    }
}

impl<T: fmt::Debug> fmt::Debug for SpannedKey<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

impl<T> Deref for SpannedKey<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}
