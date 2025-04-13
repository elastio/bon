use bon::{bon, builder, Builder};

#[builder(const)]
fn const_on_non_const_fn() {}

struct Sut;

#[bon]
impl Sut {
    #[builder(const)]
    fn const_on_non_const_fn() {}
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleDefault {
    #[builder(default)]
    x1: u32,
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleSkip {
    #[builder(skip)]
    x1: u32,
}

#[derive(Builder)]
#[builder(const)]
struct IncompatibleDefaultExpression {
    #[builder(default = return 1)]
    x1: u32,
}

fn main() {}
