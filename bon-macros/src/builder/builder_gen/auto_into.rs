use super::RegularMember;
use crate::util::prelude::*;
use darling::ast::GenericParamExt;
use std::collections::BTreeSet;

impl super::BuilderGenCtx {
    // XXX: this behavior is heavily documented in `into-conversions.md`. Please
    // keep the docs and the implementation in sync.
    pub(crate) fn member_qualifies_for_into(
        &self,
        member: &RegularMember,
        ty: &syn::Type,
    ) -> Result<bool> {
        // User override takes the wheel entirely
        let Some(user_override) = &member.params.into else {
            return Ok(self.type_qualifies_for_into(ty));
        };

        let override_value = user_override.as_ref().value;
        let default_value = self.type_qualifies_for_into(ty);

        if default_value != override_value {
            // Override makes sense since it changes the default behavior
            return Ok(override_value);
        }

        let maybe_qualifies = if default_value {
            "qualifies"
        } else {
            "doesn't qualify"
        };

        let member_origin = &member.origin;

        bail!(
            &user_override.span(),
            "This attribute is redundant and can be removed. By default the \
            the type of this {member_origin} already {maybe_qualifies} for `impl Into`.",
        );
    }

    pub(crate) fn type_qualifies_for_into(&self, ty: &syn::Type) -> bool {
        // Only simple type paths qualify for `impl Into`
        let Some(path) = ty.as_path() else {
            return false;
        };

        // <Ty as Trait>::Path projection is too complex
        if path.qself.is_some() {
            return false;
        }

        // Types with generic parameters don't qualify
        let has_generic_params = path
            .path
            .segments
            .iter()
            .any(|segment| !segment.arguments.is_empty());

        if has_generic_params {
            return false;
        }

        // Bare reference to the type parameter in scope doesn't qualify
        if let Some(ident) = path.path.get_ident() {
            let type_params: BTreeSet<_> = self
                .generics
                .params
                .iter()
                .filter_map(|param| Some(&param.as_type_param()?.ident))
                .collect();

            if type_params.contains(ident) {
                return false;
            }
        };

        // Do the check for primitive types as the last step to handle the case
        // when a generic type param was named exactly as one of the primitive types
        let primitive_types = [
            "bool", "char", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16",
            "u32", "u64", "u128", "usize",
        ];

        primitive_types.iter().all(|primitive| {
            // We check for the last segment name because primitive types may also be referenced
            // via `std::primitive::{name}` path.
            !path.path.ends_with_segment(primitive)
        })
    }
}
