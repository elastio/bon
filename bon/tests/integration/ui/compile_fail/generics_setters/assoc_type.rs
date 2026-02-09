use bon::Builder;

// Test that conversion methods properly detect generic parameter usage in associated types
// and don't allow converting the type after the field has been set.

trait MyTrait {
    type Assoc;
}

impl MyTrait for () {
    type Assoc = u32;
}

impl MyTrait for bool {
    type Assoc = u64;
}

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct AssocTypeField<T: MyTrait> {
    value: T::Assoc,
}

fn main() {
    // Test T::Assoc - can't change type after setting field
    AssocTypeField::<()>::builder()
        .value(42)
        .conv_t::<bool>()
        .build();
}
