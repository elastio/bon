use bon::Builder;

// Test that conversion methods properly handle complex interdependent bounds
// and don't allow converting the type after the field has been set.

#[derive(Builder)]
#[builder(generics(setters(name = "conv_{}")))]
struct Sut<T, U>
where
    T: IntoIterator<Item = U>,
    T::Item: Clone,
    U: Clone,
{
    value: T,
}

fn main() {
}
