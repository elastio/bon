use crate::prelude::*;

#[test]
fn smoke() {
    #[derive(Builder)]
    struct Sut {
        #[builder(pos = start_fn)]
        starter_1: bool,
        starter_2: char,

        named: u32,

        #[builder(pos = finish_fn)]
        finisher_1: &'static str,

        #[builder(pos = finish_fn)]
        finisher_2: &'static str,
    }

    Sut::builder(true, 'c')
        .named(99)
        .build("1", "2");
}
