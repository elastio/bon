mod match_types;

use crate::util::prelude::*;

pub(crate) trait TypeExt {
    /// Try downcasting the type to [`syn::Type::Path`]
    fn as_path(&self) -> Option<&syn::TypePath>;

    /// Returns the last identifier of the path if this type is a simple path
    fn last_path_segment_ident(&self) -> Option<&syn::Ident>;

    /// Returns `true` if the given type is p [`syn::Type::Path`] and its
    /// final segment is equal to `needle` identifier.
    fn is_last_segment(&self, needle: &str) -> bool;

    /// Detects if the type is `desired_type` and returns its generic type parameter
    fn type_param(&self, desired_type: &str) -> Option<&syn::Type>;

    /// Detects if the type is [`Option`] and returns its generic type parameter
    fn option_type_param(&self) -> Option<&syn::Type>;

    /// Heuristically detects if the type is [`Option`]
    fn is_option(&self) -> bool;

    /// Recursively strips the [`syn::Type::Group`] and [`syn::Type::Paren`] wrappers
    fn peel(&self) -> &Self;

    /// Returns `true` if the given type matches the pattern. The types match only if
    /// their tokens are equal or if they differ in the places where the pattern has
    /// a wildcard [`syn::Type::Infer`] e.g. `Vec<i32>` matches the pattern `Vec<_>`.
    ///
    /// Any wildcards in `Self` will not be specially handled. Only wildcards in `pattern`
    /// have semantic meaning.
    fn matches(&self, pattern: &syn::Type) -> Result<bool>;
}

impl TypeExt for syn::Type {
    fn as_path(&self) -> Option<&syn::TypePath> {
        match self.peel() {
            Self::Path(path) => Some(path),
            _ => None,
        }
    }

    fn last_path_segment_ident(&self) -> Option<&syn::Ident> {
        Some(&self.as_path()?.path.segments.last()?.ident)
    }

    fn is_last_segment(&self, needle: &str) -> bool {
        let path = match self.as_path() {
            Some(path) => path,
            _ => return false,
        };

        let last_segment = &path
            .path
            .segments
            .last()
            .expect("BUG: empty path is not possible")
            .ident;

        last_segment == needle
    }

    fn type_param(&self, desired_type: &str) -> Option<&syn::Type> {
        let path = self.as_path()?;

        let segment = path
            .path
            .segments
            .iter()
            .find(|&segment| segment.ident == desired_type)?;

        let args = match &segment.arguments {
            syn::PathArguments::AngleBracketed(args) => args,
            _ => return None,
        };

        let arg = args.args.first()?;

        let arg = match arg {
            syn::GenericArgument::Type(arg) => arg,
            _ => return None,
        };

        Some(arg)
    }

    fn option_type_param(&self) -> Option<&syn::Type> {
        self.type_param("Option")
    }

    fn is_option(&self) -> bool {
        self.is_last_segment("Option")
    }

    fn peel(&self) -> &Self {
        match self {
            Self::Group(group) => group.elem.peel(),
            Self::Paren(paren) => paren.elem.peel(),
            _ => self,
        }
    }

    fn matches(&self, pattern: &syn::Type) -> Result<bool> {
        match_types::match_types(self, pattern)
    }
}
