use crate::prelude::*;

#[test]
fn smoke_fn() {
    #[builder(derive(Clone, Debug))]
    fn sut(_arg1: bool, _arg2: Option<()>, _arg3: Option<&str>, _arg4: Option<u32>) {}

    let actual = sut().arg1(true).arg3("value").maybe_arg4(None).clone();

    assert_debug_eq(
        actual,
        expect![[r#"
            SutBuilder {
                _arg1: true,
                _arg3: Some(
                    "value",
                ),
                _arg4: None,
            }"#]],
    );
}

#[test]
fn smoke_struct() {
    #[derive(Builder)]
    #[builder(derive(Clone, Debug))]
    struct Sut<'a> {
        _arg1: bool,
        _arg2: Option<()>,
        _arg3: Option<&'a str>,
        _arg4: Option<u32>,
    }

    let actual = Sut::builder()
        .arg1(true)
        .arg3("value")
        .maybe_arg4(None)
        .clone();

    assert_debug_eq(
        actual,
        expect![[r#"
            SutBuilder {
                _arg1: true,
                _arg3: Some(
                    "value",
                ),
                _arg4: None,
            }"#]],
    );
}

#[test]
fn builder_with_receiver() {
    #[derive(Clone, Debug)]
    struct Sut {
        #[allow(dead_code)]
        name: &'static str,
    }

    #[bon]
    impl Sut {
        #[builder(derive(Clone, Debug))]
        fn method(&self, other_name: &'static str, values: &[u8]) {
            let _ = (self, other_name, values);
        }
    }

    let actual = Sut { name: "Blackjack" }
        .method()
        .other_name("P21")
        .values(&[1, 2, 3])
        .clone();

    assert_debug_eq(
        actual,
        expect![[r#"
            SutMethodBuilder {
                self: Sut {
                    name: "Blackjack",
                },
                other_name: "P21",
                values: [
                    1,
                    2,
                    3,
                ],
            }"#]],
    );
}

#[test]
fn empty_derives() {
    #[derive(Builder)]
    #[builder(derive())]
    struct Sut {
        _arg1: bool,
    }

    let _ = Sut::builder().arg1(true).build();
}

#[test]
fn skipped_members() {
    struct NoDebug;

    #[derive(Builder)]
    #[builder(derive(Debug, Clone))]
    struct Sut {
        _arg1: bool,

        #[builder(skip = NoDebug)]
        _arg2: NoDebug,
    }

    #[allow(clippy::redundant_clone)]
    let actual = Sut::builder().arg1(true).clone();

    assert_debug_eq(actual, expect!["SutBuilder { _arg1: true }"]);
}

#[test]
fn empty_builder() {
    #[derive(Builder)]
    #[builder(derive(Clone, Debug))]
    struct Sut {}

    #[allow(clippy::redundant_clone)]
    let actual = Sut::builder().clone();

    assert_debug_eq(actual, expect!["SutBuilder"]);
}
