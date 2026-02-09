use bon::Builder;

// Test that conversion methods properly detect generic parameter usage in function pointers
// and don't allow converting the type after the field has been set.

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct FnPointerField<T> {
    value: fn(T) -> T,
}

fn main() {
    // Test fn(T) -> T - can't change type after setting field
    FnPointerField::<()>::builder()
        .value(|x| x)
        .conv_t::<bool>()
        .build();
}
