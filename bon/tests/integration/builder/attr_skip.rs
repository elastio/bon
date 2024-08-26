use crate::prelude::*;
use expect_test::expect;

#[cfg(feature = "alloc")]
#[test]
fn fn_alloc() {
    #[builder]
    fn sut(
        #[builder(skip = "skip".to_owned())] arg1: String,
        #[builder(skip = vec![42])] arg2: Vec<u32>,
    ) -> (String, Vec<u32>) {
        (arg1, arg2)
    }

    assert_debug_eq(sut().call(), expect![[r#"("skip", [42])"#]]);
}

#[cfg(feature = "alloc")]
#[test]
fn struct_alloc() {
    use bon::bon;
    use expect_test::expect;

    #[builder]
    #[derive(Debug)]
    struct Sut {
        #[builder(skip = "skip".to_owned())]
        arg1: String,

        #[builder(skip = vec![42])]
        arg2: Vec<u32>,
    }

    assert_debug_eq(
        Sut::builder().build(),
        expect![[r#"Sut { arg1: "skip", arg2: [42] }"#]],
    );

    #[bon]
    impl Sut {
        #[builder]
        fn assoc(
            self,
            #[builder(skip = "assoc".to_owned())] arg1: String,
            #[builder(skip = vec![43])] arg2: Vec<u32>,
        ) -> Self {
            Self {
                arg1: format!("{}+{arg1}", self.arg1),
                arg2: self.arg2.into_iter().chain(arg2).collect(),
            }
        }
    }

    let sut = || Sut::builder().build();

    assert_debug_eq(
        sut().assoc().call(),
        expect![[r#"Sut { arg1: "skip+assoc", arg2: [42, 43] }"#]],
    );
}

#[test]
fn fn_no_std() {
    #[builder]
    fn sut(#[builder(skip)] arg1: u32, #[builder(skip = 42)] arg2: u32) -> (u32, u32) {
        (arg1, arg2)
    }

    assert_debug_eq(sut().call(), expect!["(0, 42)"]);
}

#[test]
fn struct_no_std() {
    #[builder]
    #[derive(Debug)]
    struct Sut {
        #[builder(skip)]
        arg1: u32,

        #[builder(skip = 42)]
        arg2: u32,
    }

    assert_debug_eq(Sut::builder().build(), expect!["Sut { arg1: 0, arg2: 42 }"]);

    #[bon]
    impl Sut {
        #[builder]
        fn assoc(self, #[builder(skip)] arg1: u32, #[builder(skip = 43)] arg2: u32) -> Self {
            Self {
                arg1: self.arg1 + arg1,
                arg2: self.arg2 + arg2,
            }
        }
    }

    assert_debug_eq(
        Sut::builder().build().assoc().call(),
        expect!["Sut { arg1: 0, arg2: 85 }"],
    );
}

#[test]
fn func_with_non_skipped_arg() {
    #[builder]
    fn sut(#[builder(skip)] arg1: u32, arg2: u32) -> (u32, u32) {
        (arg1, arg2)
    }

    assert_debug_eq(sut().arg2(42).call(), expect!["(0, 42)"]);
}

#[test]
fn struct_with_non_skipped_arg() {
    #[builder]
    #[derive(Debug)]
    struct Sut {
        #[builder(skip)]
        #[allow(dead_code)]
        arg1: u32,

        #[allow(dead_code)]
        arg2: u32,
    }

    assert_debug_eq(
        Sut::builder().arg2(24).build(),
        expect!["Sut { arg1: 0, arg2: 24 }"],
    );

    #[bon]
    impl Sut {
        #[builder]
        fn assoc(#[builder(skip)] arg1: u32, arg2: u32) -> Self {
            Self { arg1, arg2 }
        }
    }

    assert_debug_eq(
        Sut::assoc().arg2(42).call(),
        expect!["Sut { arg1: 0, arg2: 42 }"],
    );
}

#[test]
fn fn_generic_skipped() {
    #[builder]
    fn sut(
        #[builder(skip)] arg1: impl Clone + Default,
        #[builder(skip = <_>::default())] arg2: impl Clone + Default,
    ) {
        drop(arg1);
        drop(arg2);
    }

    sut::<(), ()>().call()
}
