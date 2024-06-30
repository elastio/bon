use prox::prelude::*;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;

pub(crate) fn normalize_fn_item(mut fn_item: syn::ItemFn) -> syn::ItemFn {
    normalize_references(&mut fn_item.sig);
    normalize_impl_traits(&mut fn_item.sig);
    fn_item
}

fn normalize_references(signature: &mut syn::Signature) {
    let mut visitor = NormalizeReferences::default();

    for arg in &mut signature.inputs {
        visitor.visit_type_mut(arg.ty_mut());
    }

    for i in 0..visitor.total_anon_lifetimes {
        let lifetime = NormalizeReferences::make_lifetime_with_index(i);
        let lifetime = syn::LifetimeParam::new(lifetime);
        let lifetime = syn::GenericParam::Lifetime(lifetime);

        // Unfortunately, there is no `spilce` in `Punctuated`, so we just do
        // a dumb insert in a loop.
        signature.generics.params.insert(i, lifetime);
    }
}

#[derive(Default)]
struct NormalizeReferences {
    total_anon_lifetimes: usize,
}

impl VisitMut for NormalizeReferences {
    fn visit_lifetime_mut(&mut self, lifetime: &mut syn::Lifetime) {
        syn::visit_mut::visit_lifetime_mut(self, lifetime);

        if lifetime.ident == "_" {
            *lifetime = self.make_lifetime();
        }
    }

    fn visit_type_reference_mut(&mut self, reference: &mut syn::TypeReference) {
        syn::visit_mut::visit_type_reference_mut(self, reference);
        reference
            .lifetime
            .get_or_insert_with(|| self.make_lifetime());
    }
}

impl NormalizeReferences {
    /// Make a lifetime with the next index. It's used to generate unique
    /// lifetimes for every occurrence of a reference with the anonymous
    /// lifetime.
    fn make_lifetime(&mut self) -> syn::Lifetime {
        let index = self.total_anon_lifetimes;
        self.total_anon_lifetimes += 1;
        Self::make_lifetime_with_index(index)
    }

    fn make_lifetime_with_index(index: usize) -> syn::Lifetime {
        let symbol = format!("'__b{index}");
        syn::Lifetime::new(&symbol, proc_macro2::Span::call_site())
    }
}

fn normalize_impl_traits(signature: &mut syn::Signature) {
    let mut visitor = NormalizeImplTraits::default();

    for arg in &mut signature.inputs {
        visitor.visit_type_mut(arg.ty_mut());
    }

    if visitor.impl_traits.is_empty() {
        return;
    }

    let new_generic_params = (0..visitor.impl_traits.len()).map(|i| {
        let ident = NormalizeImplTraits::make_type_param_ident(i);
        syn::GenericParam::Type(syn::parse_quote!(#ident))
    });

    signature.generics.params.extend(new_generic_params);

    let new_predicates =
        visitor
            .impl_traits
            .into_iter()
            .enumerate()
            .map(|(i, bounds)| -> syn::WherePredicate {
                let ident = NormalizeImplTraits::make_type_param_ident(i);
                syn::parse_quote!(#ident: #bounds)
            });

    signature
        .generics
        .make_where_clause()
        .predicates
        .extend(new_predicates);
}

#[derive(Default)]
struct NormalizeImplTraits {
    impl_traits: Vec<Punctuated<syn::TypeParamBound, syn::Token![+]>>,
}

impl VisitMut for NormalizeImplTraits {
    fn visit_type_mut(&mut self, ty: &mut syn::Type) {
        syn::visit_mut::visit_type_mut(self, ty);

        if !matches!(ty, syn::Type::ImplTrait(_)) {
            return;
        };

        let type_param = Self::make_type_param_ident(self.impl_traits.len());
        let impl_trait = std::mem::replace(ty, syn::Type::Path(syn::parse_quote!(#type_param)));

        let syn::Type::ImplTrait(impl_trait) = impl_trait else {
            panic!("BUG: code higher validated that this is impl trait: {impl_trait:?}",)
        };

        self.impl_traits.push(impl_trait.bounds);
    }
}

impl NormalizeImplTraits {
    fn make_type_param_ident(index: usize) -> syn::Ident {
        quote::format_ident!("__{index}")
    }
}

/// Remove all doc comments attributes from function arguments, because they are
/// not valid in that position in regular Rust code. The cool trick is that they
/// are still valid syntactically when a proc macro like this one pre-processes
/// them and removes them from the expanded code. We use the doc comments to put
/// them on the generated setter methods.
pub(crate) fn strip_doc_comments_from_args(sig: &mut syn::Signature) {
    for arg in &mut sig.inputs {
        arg.attrs_mut().retain(|attr| !attr.is_doc());
    }
}
