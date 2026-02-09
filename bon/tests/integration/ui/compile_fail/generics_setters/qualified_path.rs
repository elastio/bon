use bon::Builder;

// Test that conversion methods properly detect generic parameter usage in qualified paths
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
struct QualifiedPathField<T: MyTrait> {
    value: <T as MyTrait>::Assoc,
}

fn main() {
    // Test <T as Trait>::Assoc - can't change type after setting field
    QualifiedPathField::<()>::builder()
        .value(42)
        .conv_t::<bool>()
        .build();
}
