use bon::Builder;

// Test that conversion methods properly detect generic parameter usage in raw pointers
// and don't allow converting the type after the field has been set.

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct RawPointerField<T> {
    value: *const T,
}

fn main() {
    // Test *const T - can't change type after setting field
    RawPointerField::<()>::builder()
        .value(core::ptr::null())
        .conv_t::<bool>()
        .build();
}
