#![deny(warnings)]

struct Sut;

#[bon::bon]
impl Sut {
    #[builder]
    fn sut(self, #[builder(start_fn)] _x1: u32, _x2: u32) {}
}

fn main() {
    let sut = Sut.sut(99);

    let SutSutBuilder {
        __unsafe_private_phantom: _,
        __unsafe_private_start_fn_args: _,
        __unsafe_private_receiver: _,
        __unsafe_private_named: _,
    } = sut;
}
