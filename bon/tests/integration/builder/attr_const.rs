use crate::prelude::*;

#[test]
const fn smoke_struct() {
    #[derive(Builder)]
    #[builder(const)]
    struct Sut {
        x1: u32,

        x2: Option<u32>,

        #[builder(default = x1 + 99)]
        x3: u32,

        #[builder(skip = 4)]
        x4: u32,

        #[builder(with = |a: u32, b: u32| a + b)]
        x5: u32,

        #[builder(with = |a: u32, b: u32| -> Result<_, ()> { Ok(a + b) })]
        _x6: Option<u32>,
        //
        // This doesn't work because Rust complains about this in setters that
        // consume `self` and return a new instance of the builder:
        // ```
        // destructor of `builder::attr_const::smoke_struct::SutBuilder<S>`
        // cannot be evaluated at compile-time
        // ```
        // x7: Vec<String>,
        // x8: String,
    }

    const ACTUAL: Sut = Sut::builder().x1(2).x2(2).x5(3, 4).build();

    #[allow(clippy::assertions_on_constants)]
    {
        assert!(ACTUAL.x1 == 2);
        assert!(ACTUAL.x2.unwrap() == 2);
        assert!(ACTUAL.x3 == 101);
        assert!(ACTUAL.x4 == 4);
        assert!(ACTUAL.x5 == 7);
    }
}
