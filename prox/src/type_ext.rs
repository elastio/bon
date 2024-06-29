use easy_ext::ext;

#[ext(TypeExt)]
pub impl syn::Type {
    /// Returns `true` if the given type is p [`syn::Type::Path`] and its
    /// final segment is equal to `needle` identifier.
    fn is_final_segment(&self, needle: &str) -> bool {
        if let syn::Type::Path(path) = self.strip_group() {
            let last_segment = &path
                .path
                .segments
                .last()
                .expect("BUG: empty path is not possible")
                .ident;

            return last_segment == needle;
        }
        false
    }

    /// Detects if the type is `desired_type` and returns the underlying generic type
    fn item_ty(&self, desired_type: &str) -> Option<&syn::Type> {
        let syn::Type::Path(path) = self.strip_group() else {
            return None;
        };

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

    /// Detects if the type is [`Vec`] and returns its items type
    fn vec_item_ty(&self) -> Option<&syn::Type> {
        self.item_ty("Vec")
    }

    /// Detects if the type is [`Option`] and returns its underlying value type
    fn option_item_ty(&self) -> Option<&syn::Type> {
        self.item_ty("Option")
    }

    /// Detects if the type is [`Box`] and returns its underlying value type
    fn box_item_ty(&self) -> Option<&syn::Type> {
        self.item_ty("Box")
    }

    /// Heuristically detects if the type is [`Option`]
    fn is_option(&self) -> bool {
        self.is_final_segment("Option")
    }

    /// Heuristically detects if the type is [`std::collections::HashSet`]
    fn is_hash_set(&self) -> bool {
        self.is_final_segment("HashSet")
    }

    /// Heuristically detects if the type is [`std::collections::BTreeSet`]
    fn is_btree_set(&self) -> bool {
        self.is_final_segment("BTreeSet")
    }

    /// Heuristically detects if the type is one of the following map types:
    /// - [`std::collections::HashMap`]
    /// - [`std::collections::BTreeMap`]
    fn is_map(&self) -> bool {
        self.is_hash_map() || self.is_btree_map()
    }

    /// Heuristically detects if the type is [`std::collections::HashMap`]
    fn is_hash_map(&self) -> bool {
        self.is_final_segment("HashMap")
    }

    /// Heuristically detects if the type is [`std::collections::BTreeMap`]
    fn is_btree_map(&self) -> bool {
        self.is_final_segment("BTreeMap")
    }

    /// Heuristically detects if the type is [`Vec`]
    fn is_vec(&self) -> bool {
        self.is_final_segment("Vec")
    }

    /// Heuristically detects if the type is [`String`]
    fn is_string(&self) -> bool {
        self.is_final_segment("String")
    }

    /// Heuristically detects if the type is [`bool`]
    fn is_bool(&self) -> bool {
        self.is_final_segment("bool")
    }

    /// Recursively strips the [`syn::Type::Group`] wrappers
    fn strip_group(&self) -> &Self {
        match self {
            Self::Group(group) => group.elem.strip_group(),
            _ => self,
        }
    }
}
