use crate::prelude::*;
use core::fmt;

#[test]
fn test_assoc_method() {
    {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder(start_fn = builder)]
            fn new() -> Self {
                Self
            }

            #[builder(start_fn = regular_builder)]
            fn regular() {}
        }

        let _: Sut = Sut::builder().build();
        let _: Sut = Sut::new();

        Sut::regular_builder().call();
        Sut::regular();
    }

    {
        struct Sut;

        #[bon]
        impl Sut {
            #[builder(start_fn(name = builder, vis = ""))]
            pub(crate) fn new() -> Self {
                Self
            }

            #[builder(start_fn(name = regular_builder, vis = ""))]
            pub(crate) fn regular() {}
        }

        let _: Sut = Sut::builder().build();
        let _: Sut = Sut::new();

        Sut::regular_builder().call();
        Sut::regular();
    }
}

#[test]
fn test_free_fn() {
    {
        #[builder(start_fn(name = sut_builder))]
        fn sut(arg1: bool, arg2: u32) -> impl fmt::Debug {
            (arg1, arg2)
        }

        assert_debug_eq(sut(true, 42), expect!["(true, 42)"]);
    }

    {
        #[builder(start_fn = sut_builder)]
        fn sut(arg1: u32) -> u32 {
            arg1
        }

        assert_debug_eq(sut(42), expect!["42"]);
    }

    {
        #[builder(start_fn(name = sut_builder, vis = ""))]
        fn sut(arg1: u32) -> u32 {
            arg1
        }

        assert_debug_eq(sut(42), expect!["42"]);
    }

    {
        /// Docs on `sut`
        #[builder(start_fn(name = sut_builder, doc {
            /// Docs on `sut_builder`
        }))]
        fn sut(arg1: u32) -> u32 {
            arg1
        }

        assert_debug_eq(sut_builder().arg1(42).call(), expect!["42"]);
        assert_debug_eq(sut(42), expect!["42"]);
    }
}
