use crate::prelude::*;

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
fn builder_start_fn_special_case() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder(start_fn = builder)]
        fn some_other_name() {}
    }

    let _: SutBuilder = Sut::builder();
    let builder: SutBuilder<sut_builder::Empty> = Sut::builder();

    builder.build();

    Sut::some_other_name();
}
