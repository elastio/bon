use bon::Builder;

// Test that conversion methods properly detect generic parameter usage in arrays
// and don't allow converting the type after the field has been set.

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct ArrayField<T> {
    value: [T; 2],
}

fn main() {
    // Test [T; N] - can't change type after setting field
    ArrayField::<()>::builder()
        .value([(), ()])
        .conv_t::<bool>()
        .build();
}
