#[rustversion::since(1.61.0)]

mod msrv_1_61 {

    mod smoke {

        use crate::prelude::*;

        #[test]
        const fn test_struct() {
            #[derive(Builder)]
            #[builder(const)]
            struct Sut {
                #[builder(start_fn)]
                x1: u32,

                #[builder(finish_fn)]
                x2: u32,

                x3: u32,

                x4: Option<u32>,

                #[builder(default = x1 + 99)]
                x5: u32,

                #[builder(with = |a: u32, b: u32| a + b)]
                x6: u32,

                #[builder(with = |a: u32, b: u32| -> Result<_, ()> { Ok(a + b) })]
                x7: Option<u32>,

                #[builder(skip = 10)]
                x8: u32,
                //
                // This doesn't work because Rust complains about this in setters that
                // consume `self` and return a new instance of the builder:
                // ```
                // destructor of `builder::attr_const::test_struct::SutBuilder<S>`
                // cannot be evaluated at compile-time
                // ```
                // x7: Vec<String>,
                // x8: String,
            }

            const ACTUAL: Sut = Sut::builder(1).x3(2).x4(3).x6(4, 5).build(6);

            #[allow(clippy::assertions_on_constants)]
            {
                assert!(ACTUAL.x1 == 1);
                assert!(ACTUAL.x2 == 6);
                assert!(ACTUAL.x3 == 2);
                assert!(matches!(ACTUAL.x4, Some(3)));
                assert!(ACTUAL.x5 == 100);
                assert!(ACTUAL.x6 == 9);
                assert!(ACTUAL.x7.is_none());
                assert!(ACTUAL.x8 == 10);
            }
        }
        #[test]
        const fn test_function() {
            type Output = (u32, u32, u32, Option<u32>, u32, u32, Option<u32>);

            #[builder(const)]
            const fn sut(
                #[builder(start_fn)] x1: u32,

                #[builder(finish_fn)] x2: u32,

                x3: u32,

                x4: Option<u32>,

                #[builder(default = x1 + 99)] x5: u32,

                #[builder(with = |a: u32, b: u32| a + b)] x6: u32,

                #[builder(with = |a: u32, b: u32| -> Result<_, ()> { Ok(a + b) })] x7: Option<u32>,
            ) -> Output {
                (x1, x2, x3, x4, x5, x6, x7)
            }

            const ACTUAL: Output = sut(1).x3(2).x4(3).x6(4, 5).call(6);

            #[allow(clippy::assertions_on_constants)]
            {
                assert!(ACTUAL.0 == 1);
                assert!(ACTUAL.1 == 6);
                assert!(ACTUAL.2 == 2);
                assert!(matches!(ACTUAL.3, Some(3)));
                assert!(ACTUAL.4 == 100);
                assert!(ACTUAL.5 == 9);
                assert!(ACTUAL.6.is_none());
            }
        }

        #[test]
        const fn test_method() {
            type Output = (u32, u32, u32, Option<u32>, u32, u32, Option<u32>);

            struct Sut;

            #[bon]
            impl Sut {
                #[builder(const)]
                const fn sut(
                    #[builder(start_fn)] x1: u32,

                    #[builder(finish_fn)] x2: u32,

                    x3: u32,

                    x4: Option<u32>,

                    #[builder(default = x1 + 99)] x5: u32,

                    #[builder(with = |a: u32, b: u32| a + b)] x6: u32,

                    #[builder(with = |a: u32, b: u32| -> Result<_, ()> { Ok(a + b) })] x7: Option<
                        u32,
                    >,
                ) -> Output {
                    (x1, x2, x3, x4, x5, x6, x7)
                }
            }

            const ACTUAL: Output = Sut::sut(1).x3(2).x4(3).x6(4, 5).call(6);

            #[allow(clippy::assertions_on_constants)]
            {
                assert!(ACTUAL.0 == 1);
                assert!(ACTUAL.1 == 6);
                assert!(ACTUAL.2 == 2);
                assert!(matches!(ACTUAL.3, Some(3)));
                assert!(ACTUAL.4 == 100);
                assert!(ACTUAL.5 == 9);
                assert!(ACTUAL.6.is_none());
            }
        }
    }

    // Tests for the following bug: https://github.com/elastio/bon/issues/287
    mod visibility {
        use crate::prelude::*;

        #[test]
        const fn test_function() {
            #[builder(const)]
            pub const fn sut(#[builder(getter)] _x: u32) {}

            sut().x(1).call();
        }

        #[test]
        const fn test_method() {
            struct Sut;

            #[bon]
            impl Sut {
                #[builder(const)]
                pub const fn sut(#[builder(getter)] _x: u32) {}
            }

            Sut::sut().x(1).call();
        }
    }
}
