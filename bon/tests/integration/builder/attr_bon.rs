use crate::prelude::*;

#[test]
fn new_method_special_case() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder]
        fn new() {}
    }

    let _: SutBuilder = Sut::builder();
    let builder: SutBuilder<sut_builder::Empty> = Sut::builder();

    builder.build();
}

#[test]
fn builder_method_special_case() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder]
        fn builder() {}
    }

    let _: SutBuilder = Sut::builder();
    let builder: SutBuilder<sut_builder::Empty> = Sut::builder();

    builder.build();
}

#[test]
fn builder_start_fn_is_not_special_case() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(start_fn = builder)]
        fn some_other_name() {}
    }

    let _: SutSomeOtherNameBuilder = Sut::builder();
    let builder: SutSomeOtherNameBuilder<sut_some_other_name_builder::Empty> = Sut::builder();

    builder.call();

    Sut::some_other_name();
}
