use bon::Builder;

// Test that conversion methods reject bounds that reference other type parameters

#[derive(Builder)]
#[builder(generics(setters(name = "with_{}")))]
struct Sut<Iter: Iterator<Item = Item>, Item> {
    value1: Iter,
}

fn main() {
}
