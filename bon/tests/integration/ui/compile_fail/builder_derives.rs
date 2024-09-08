use bon::{bon, builder, Builder};

struct NoTraitImpls;

#[derive(Builder)]
#[builder(derive(Clone, Debug))]
struct StructContainsNonTrait {
    non_debug: NoTraitImpls,
    x: u32,
}

#[builder(derive(Clone, Debug))]
fn fn_contains_non_trait(_non_debug: NoTraitImpls, _x: u32) {}

#[bon]
impl StructContainsNonTrait {
    #[builder(derive(Clone, Debug))]
    fn method_contains_non_trait(_non_debug: NoTraitImpls, _x: u32) {}
}

fn main() {}
