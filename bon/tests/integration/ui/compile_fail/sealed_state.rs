use bon::Builder;

#[derive(Builder)]
struct Sut {
    x1: u32,
}

struct CustomState;

impl sut_builder::State for CustomState {
    type X1 = sut_builder::SetX1;
    type __Sealed = ();
}

fn main() {}
