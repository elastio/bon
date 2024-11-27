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
        __private_phantom: _,
        __private_start_fn_args: _,
        __private_receiver: _,
        __private_named: _,
    } = sut;
}
