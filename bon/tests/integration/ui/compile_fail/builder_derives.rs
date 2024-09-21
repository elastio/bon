use bon::{bon, builder, Builder};

struct NoTraitImpls;

#[derive(Builder)]
#[builder(derive(Clone, Debug))]
struct StructContainsNonTrait {
    no_impls: NoTraitImpls,

    no_impl_optional: Option<NoTraitImpls>,

    #[builder(default = NoTraitImpls)]
    no_impl_optional_2: NoTraitImpls,

    x: u32,
}

#[builder(derive(Clone, Debug))]
fn fn_contains_non_trait(
    _no_impls: NoTraitImpls,

    _no_impl_optional: Option<NoTraitImpls>,

    #[builder(default = NoTraitImpls)] //
    _no_impl_optional_2: NoTraitImpls,

    _x: u32,
) {
}

#[bon]
impl StructContainsNonTrait {
    #[builder(derive(Clone, Debug))]
    fn method_contains_non_trait(
        _no_impls: NoTraitImpls,

        _no_impl_optional: Option<NoTraitImpls>,

        #[builder(default = NoTraitImpls)] //
        _no_impl_optional_2: NoTraitImpls,

        _x: u32,
    ) {
    }
}

fn main() {}
