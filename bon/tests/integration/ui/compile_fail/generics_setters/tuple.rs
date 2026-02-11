use bon::Builder;

// Test that conversion methods properly detect generic parameter usage in tuples
// and don't allow converting the type after the field has been set.

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct TupleField<T> {
    value: (T, T),
}

fn main() {
    // Test (T, T) - can't change type after setting field
    TupleField::<()>::builder()
        .value(((), ()))
        .conv_t::<bool>()
        .build();
}
