use crate::prelude::*;

#[test]
fn method_new_doesnt_require_a_value_for_name() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(expose_positional_fn)]
        fn new() -> Self {
            Self
        }
    }

    let _: Sut = Sut::builder().build();
    let _: Sut = Sut::new();

    #[allow(clippy::items_after_statements)]
    struct Sut2;

    #[bon]
    impl Sut2 {
        #[builder(expose_positional_fn(vis = "pub(crate)"))]
        fn new() -> Self {
            Self
        }
    }

    let _: Sut2 = Sut2::builder().build();
    let _: Sut2 = Sut2::new();
}

#[test]
fn with_nested_params() {
    #[builder(expose_positional_fn(name = positional))]
    fn sut(arg1: bool, arg2: u32) -> (bool, u32) {
        (arg1, arg2)
    }

    assert_debug_eq(positional(true, 42), expect!["(true, 42)"]);
}

#[test]
fn simple() {
    #[builder(expose_positional_fn = positional)]
    fn sut(arg1: u32) -> u32 {
        arg1
    }

    assert_debug_eq(positional(42), expect!["42"]);
}
