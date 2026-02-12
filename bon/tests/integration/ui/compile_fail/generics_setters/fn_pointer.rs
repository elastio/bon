use bon::Builder;

// Test that conversion methods properly detect generic parameter usage in function pointers
// and don't allow converting the type after the field has been set.

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct FnPointerFieldInOut<T> {
    value: fn(T) -> T,
}

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct FnPointerFieldIn<T> {
    value: fn(T),
}

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct FnPointerFieldOut<T> {
    value: fn() -> T,
}

fn main() {
    FnPointerFieldInOut::<()>::builder()
        .value(|()| ())
        .conv_t::<bool>()
        .build();

    FnPointerFieldIn::<()>::builder()
        .value(|()| ())
        .conv_t::<bool>()
        .build();

    FnPointerFieldOut::<()>::builder()
        .value(|| ())
        .conv_t::<bool>()
        .build();
}
