use bon::{bon, builder};

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

    let actual = positional(true, 42);
    assert_eq!(actual, (true, 42));
}

#[test]
fn simple() {
    #[builder(expose_positional_fn = positional)]
    fn sut(arg1: String) -> String {
        arg1
    }

    assert_eq!(positional("arg1".to_owned()), "arg1");
}
