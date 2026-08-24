use super::GenericsNamespace;
use crate::util::prelude::*;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;

pub(crate) struct NormalizeLifetimes<'a> {
    namespace: &'a GenericsNamespace,
}

impl<'a> NormalizeLifetimes<'a> {
    pub(crate) fn new(namespace: &'a GenericsNamespace) -> Self {
        Self { namespace }
    }
}

impl VisitMut for NormalizeLifetimes<'_> {
    fn visit_item_impl_mut(&mut self, impl_block: &mut syn::ItemImpl) {
        for item in &mut impl_block.items {
            self.visit_impl_item_mut(item);
        }

        AssignLifetimes::new(self, "i", &mut impl_block.generics)
            .visit_type_mut(&mut impl_block.self_ty);
    }

    fn visit_impl_item_mut(&mut self, item: &mut syn::ImplItem) {
        if let syn::ImplItem::Fn(fn_item) = item {
            self.visit_signature_mut(&mut fn_item.sig);
        }
    }

    fn visit_item_fn_mut(&mut self, fn_item: &mut syn::ItemFn) {
        self.visit_signature_mut(&mut fn_item.sig);
    }

    fn visit_signature_mut(&mut self, signature: &mut syn::Signature) {
        let mut visitor = AssignLifetimes::new(self, "f", &mut signature.generics);
        for arg in &mut signature.inputs {
            visitor.visit_fn_arg_mut(arg);
        }

        let return_type = match &mut signature.output {
            syn::ReturnType::Type(_, return_type) => return_type,
            syn::ReturnType::Default => return,
        };

        // Now perform lifetime elision for the lifetimes in the return type.
        // This code implements the logic described in the Rust reference:
        // https://doc.rust-lang.org/reference/lifetime-elision.html

        let elided_output_lifetime = signature
            .inputs
            .first()
            .and_then(|arg| {
                // Elided lifetimes in the return type are resolved to the
                // lifetime of `&'a self`, or to the one of the explicit
                // `self: &'a Self` type.
                match &arg.as_receiver()?.kind {
                    syn::ReceiverKind::Reference(_, lifetime, _) => lifetime.as_ref(),
                    syn::ReceiverKind::Typed(_, ty) => match &**ty {
                        syn::Type::Reference(reference) => reference.lifetime.as_ref(),
                        _ => None,
                    },
                    _ => None,
                }
            })
            .or_else(|| {
                let lifetime = signature
                    .inputs
                    .iter()
                    .filter_map(syn::FnArg::as_typed)
                    .fold(LifetimeCollector::None, |mut acc, pat_type| {
                        acc.visit_pat_type(pat_type);
                        acc
                    });

                match lifetime {
                    LifetimeCollector::Single(lifetime) => Some(lifetime),
                    _ => None,
                }
            });

        let elided_lifetime = match elided_output_lifetime {
            Some(elided_lifetime) => elided_lifetime,
            _ => return,
        };

        ElideOutputLifetime { elided_lifetime }.visit_type_mut(return_type);
    }
}

struct AssignLifetimes<'a> {
    base: &'a NormalizeLifetimes<'a>,

    prefix: &'static str,

    /// Generics where the assigned lifetimes should be stored.
    generics: &'a mut syn::Generics,

    next_lifetime_index: usize,
}

impl<'a> AssignLifetimes<'a> {
    fn new(
        base: &'a NormalizeLifetimes<'a>,
        prefix: &'static str,
        generics: &'a mut syn::Generics,
    ) -> Self {
        Self {
            base,
            prefix,
            generics,
            next_lifetime_index: 1,
        }
    }
}

impl VisitMut for AssignLifetimes<'_> {
    fn visit_item_mut(&mut self, _item: &mut syn::Item) {
        // Don't recurse into nested items because lifetimes aren't available there.
    }

    fn visit_type_fn_ptr_mut(&mut self, _fn_ptr: &mut syn::TypeFnPtr) {
        // Skip function pointers because anon lifetimes that appear in them
        // don't belong to the surrounding function signature.
    }

    fn visit_parenthesized_generic_arguments_mut(
        &mut self,
        _args: &mut syn::ParenthesizedGenericArguments,
    ) {
        // Skip Fn traits for the same reason as function pointers described higher.
    }

    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if lifetime.ident == "_" {
            *lifetime = self.next_lifetime();
        }
    }

    fn visit_type_reference_mut(&mut self, reference: &mut syn::TypeReference) {
        syn::visit_mut::visit_type_reference_mut(self, reference);
        reference
            .lifetime
            .get_or_insert_with(|| self.next_lifetime());
    }

    fn visit_receiver_mut(&mut self, receiver: &mut syn::Receiver) {
        match &mut receiver.kind {
            // If this is a `self: Type` syntax, then it's not a special case
            // and we can just visit the explicit type of the receiver as usual
            syn::ReceiverKind::Typed(_colon, ty) => {
                syn::visit_mut::visit_type_mut(self, ty);
            }
            syn::ReceiverKind::Reference(_and, lifetime, _mutability) => {
                if matches!(lifetime, Some(lifetime) if lifetime.ident != "_") {
                    return;
                }

                *lifetime = Some(self.next_lifetime());
            }
            _ => {}
        }
    }
}

impl AssignLifetimes<'_> {
    /// Make a lifetime with the next index. It's used to generate unique
    /// lifetimes for every occurrence of a reference with the anonymous
    /// lifetime.
    fn next_lifetime(&mut self) -> syn::Lifetime {
        let index = self.next_lifetime_index;
        self.next_lifetime_index += 1;

        let mut lifetime = self
            .base
            .namespace
            .unique_lifetime(format!("{}{index}", self.prefix));

        // `syn::Lifetime::new` requires the string to start with the `'` character,
        // which is just discarded in that method's impl 🗿.
        lifetime.insert(0, '\'');

        let lifetime = syn::Lifetime::new(&lifetime, Span::call_site());

        let lifetime_param = syn::LifetimeParam::new(lifetime.clone());
        let lifetime_param = syn::GenericParam::Lifetime(lifetime_param);
        self.generics.params.insert(index - 1, lifetime_param);

        lifetime
    }
}

enum LifetimeCollector<'a> {
    None,
    Single(&'a syn::Lifetime),
    Multiple,
}

impl<'a> Visit<'a> for LifetimeCollector<'a> {
    fn visit_item(&mut self, _item: &syn::Item) {
        // Don't recurse into nested items because lifetimes aren't available there.
    }

    fn visit_type_fn_ptr(&mut self, _fn_ptr: &syn::TypeFnPtr) {
        // Skip function pointers because anon lifetimes that appear in them
        // don't belong to the surrounding function signature.
    }

    fn visit_parenthesized_generic_arguments(
        &mut self,
        _args: &syn::ParenthesizedGenericArguments,
    ) {
        // Skip Fn traits for the same reason as function pointers described higher.
    }

    fn visit_lifetime(&mut self, lifetime: &'a syn::Lifetime) {
        match self {
            Self::None => *self = Self::Single(lifetime),
            Self::Single(_) => *self = Self::Multiple,
            Self::Multiple => {}
        }
    }
}

struct ElideOutputLifetime<'a> {
    elided_lifetime: &'a syn::Lifetime,
}

impl VisitMut for ElideOutputLifetime<'_> {
    fn visit_item_mut(&mut self, _item: &mut syn::Item) {
        // Don't recurse into nested items because lifetimes aren't available there.
    }

    fn visit_type_fn_ptr_mut(&mut self, _fn_ptr: &mut syn::TypeFnPtr) {
        // Skip function pointers because anon lifetimes that appear in them
        // don't belong to the surrounding function signature.
    }

    fn visit_parenthesized_generic_arguments_mut(
        &mut self,
        _args: &mut syn::ParenthesizedGenericArguments,
    ) {
        // Skip Fn traits for the same reason as function pointers described higher.
    }

    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        if lifetime.ident == "_" {
            *lifetime = self.elided_lifetime.clone();
        }
    }

    fn visit_type_reference_mut(&mut self, reference: &mut syn::TypeReference) {
        syn::visit_mut::visit_type_reference_mut(self, reference);

        reference
            .lifetime
            .get_or_insert_with(|| self.elided_lifetime.clone());
    }
}
