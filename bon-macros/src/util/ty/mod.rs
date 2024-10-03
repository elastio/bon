mod match_types;

use crate::util::prelude::*;
use syn::punctuated::Punctuated;

pub(crate) trait TypeExt {
    /// Try downcasting the type to [`syn::Type::Path`]
    fn as_path(&self) -> Option<&syn::TypePath>;

    /// Try downcasting the type to [`syn::Type::Path`]. If it has a [`syn::QSelf`]
    /// then this method will return `None`.
    fn as_path_no_qself(&self) -> Option<&syn::Path>;

    /// Detects if the type is [`Option`] and returns its generic type parameter
    fn option_type_param(&self) -> Option<&syn::Type>;

    /// Validates that this type is a generic type (path without [`syn::QSelf`])
    /// which ends with the given `desired_last_segment`, and returns its
    /// angle-bracketed arguments
    fn as_generic_angle_bracketed(
        &self,
        desired_last_segment: &str,
    ) -> Option<&Punctuated<syn::GenericArgument, syn::Token![,]>>;

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

    fn as_path_no_qself(&self) -> Option<&syn::Path> {
        let path = self.as_path()?;
        if path.qself.is_some() {
            return None;
        }
        Some(&path.path)
    }

    fn option_type_param(&self) -> Option<&syn::Type> {
        let args = self.as_generic_angle_bracketed("Option")?;
        if args.len() != 1 {
            return None;
        }

        let arg = args.first()?;

        let arg = match arg {
            syn::GenericArgument::Type(arg) => arg,
            _ => return None,
        };

        Some(arg)
    }

    fn as_generic_angle_bracketed(
        &self,
        desired_last_segment: &str,
    ) -> Option<&Punctuated<syn::GenericArgument, syn::Token![,]>> {
        let path = self.as_path_no_qself()?;

        let last_segment = path.segments.last()?;

        if last_segment.ident != desired_last_segment {
            return None;
        }

        match &last_segment.arguments {
            syn::PathArguments::AngleBracketed(args) => Some(&args.args),
            _ => None,
        }
    }

    fn is_option(&self) -> bool {
        self.as_path_no_qself()
            .map(|path| path.ends_with_segment("Option"))
            .unwrap_or(false)
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
