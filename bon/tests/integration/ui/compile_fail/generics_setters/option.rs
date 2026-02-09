use bon::Builder;

// Test that conversion methods properly detect generic parameter usage in Option<T>
// and don't allow converting the type after the field has been set.

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct OptionField<T> {
    value: core::option::Option<T>,
}

fn main() {
    // Test Option<T> - can't change type after setting field
    OptionField::<()>::builder()
        .value(())
        .conv_t::<bool>()
        .build();
}
