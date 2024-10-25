use bon::{bon, builder};

#[builder(crate = self::bon)]
fn relative_1() {}

#[builder(crate = super::bon)]
fn relative_2() {}

#[builder(crate = bon)]
fn relative_3() {}

struct CrateAttrInMethod;

#[bon]
impl CrateAttrInMethod {
    #[builder(crate = ::bon)]
    fn method() {}
}

struct Relative;

#[bon(crate = self::bon)]
impl Relative {
    #[builder]
    fn method1() {}
}

#[bon(crate = super::bon)]
impl Relative {
    #[builder]
    fn method2() {}
}

#[bon(crate = bon)]
impl Relative {
    #[builder]
    fn method3() {}
}



fn main() {}
