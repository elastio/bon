use crate::util::prelude::*;
use syn::visit_mut::VisitMut;

pub(crate) struct NormalizeImplTraits;

impl VisitMut for NormalizeImplTraits {
    fn visit_impl_item_fn_mut(&mut self, fn_item: &mut syn::ImplItemFn) {
        // We are interested only in signatures of functions. Don't recurse
        // into the function's block.
        self.visit_signature_mut(&mut fn_item.sig);
    }

    fn visit_signature_mut(&mut self, signature: &mut syn::Signature) {
        let mut visitor = AssignTypeParams::new(&mut signature.generics);

        for arg in &mut signature.inputs {
            visitor.visit_fn_arg_mut(arg);
        }
    }
}

struct AssignTypeParams<'a> {
    generics: &'a mut syn::Generics,
    next_type_param_index: usize,
}

impl<'a> AssignTypeParams<'a> {
    fn new(generics: &'a mut syn::Generics) -> Self {
        Self {
            generics,
            next_type_param_index: 0,
        }
    }
}

impl VisitMut for AssignTypeParams<'_> {
    fn visit_item_mut(&mut self, _item: &mut syn::Item) {
        // Don't recurse into nested items because `impl Trait` isn't available there.
    }

    fn visit_signature_mut(&mut self, signature: &mut syn::Signature) {
        for arg in &mut signature.inputs {
            self.visit_type_mut(arg.ty_mut());
        }
    }

    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        syn::visit_mut::visit_type_mut(self, ty);

        if !matches!(ty, syn::Type::ImplTrait(_)) {
            return;
        };

        let index = self.next_type_param_index;
        self.next_type_param_index += 1;

        let type_param = quote::format_ident!("__{index}");
        let impl_trait = std::mem::replace(ty, syn::Type::Path(syn::parse_quote!(#type_param)));

        let impl_trait = match impl_trait {
            syn::Type::ImplTrait(impl_trait) => impl_trait,
            _ => {
                unreachable!("BUG: code higher validated that this is impl trait: {impl_trait:?}");
            }
        };

        self.generics
            .params
            .push(syn::GenericParam::Type(syn::parse_quote!(#type_param)));

        let bounds = impl_trait.bounds;

        self.generics
            .make_where_clause()
            .predicates
            .push(syn::parse_quote!(#type_param: #bounds));
    }
}
