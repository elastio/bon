pub(crate) trait TypeExt {
    /// Try downcasting the type to [`syn::Type::Path`]
    fn as_path(&self) -> Option<&syn::TypePath>;

    /// Returns `true` if the given type is p [`syn::Type::Path`] and its
    /// final segment is equal to `needle` identifier.
    fn is_final_segment(&self, needle: &str) -> bool;

    /// Detects if the type is `desired_type` and returns its generic type parameter
    fn type_param(&self, desired_type: &str) -> Option<&syn::Type>;

    /// Detects if the type is [`Option`] and returns its generic type parameter
    fn option_type_param(&self) -> Option<&syn::Type>;

    /// Heuristically detects if the type is [`Option`]
    fn is_option(&self) -> bool;

    /// Recursively strips the [`syn::Type::Group`] and [`syn::Type::Paren`] wrappers
    fn peel(&self) -> &Self;
}

impl TypeExt for syn::Type {
    fn as_path(&self) -> Option<&syn::TypePath> {
        match self.peel() {
            syn::Type::Path(path) => Some(path),
            _ => None,
        }
    }

    fn is_final_segment(&self, needle: &str) -> bool {
        let Some(path) = self.as_path() else {
            return false;
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

        let vec_segment = path
            .path
            .segments
            .iter()
            .find(|&segment| segment.ident == desired_type)?;

        let syn::PathArguments::AngleBracketed(args) = &vec_segment.arguments else {
            return None;
        };

        let arg = args.args.first()?;

        let syn::GenericArgument::Type(arg) = arg else {
            return None;
        };

        Some(arg)
    }

    fn option_type_param(&self) -> Option<&syn::Type> {
        self.type_param("Option")
    }

    fn is_option(&self) -> bool {
        self.is_final_segment("Option")
    }

    fn peel(&self) -> &Self {
        match self {
            Self::Group(group) => group.elem.peel(),
            Self::Paren(paren) => paren.elem.peel(),
            _ => self,
        }
    }
}
