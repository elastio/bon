use crate::prelude::*;

#[cfg(feature = "alloc")]
#[test]
fn fn_alloc() {
    #[builder]
    fn sut(
        #[builder(default = "default".to_owned())] arg1: String,
        #[builder(default = vec![42])] arg2: Vec<u32>,
        #[builder(default = "foo", into)] arg3: String,
    ) -> (String, Vec<u32>, String) {
        (arg1, arg2, arg3)
    }

    assert_debug_eq(sut().call(), expect![[r#"("default", [42], "foo")"#]]);

    assert_debug_eq(
        sut().arg1("arg1".to_owned()).call(),
        expect![[r#"("arg1", [42], "foo")"#]],
    );

    assert_debug_eq(
        sut().maybe_arg1(None).call(),
        expect![[r#"("default", [42], "foo")"#]],
    );

    assert_debug_eq(
        sut().maybe_arg3(None::<String>).call(),
        expect![[r#"("default", [42], "foo")"#]],
    );
}

#[cfg(feature = "alloc")]
#[test]
fn struct_alloc() {
    use bon::bon;
    use expect_test::expect;

    #[derive(Debug, Builder)]
    struct Sut {
        #[builder(default = "default".to_owned())]
        arg1: String,

        #[builder(default = vec![42])]
        arg2: Vec<u32>,

        #[builder(default = "foo", into)]
        arg3: String,
    }

    assert_debug_eq(
        Sut::builder().build(),
        expect![[r#"Sut { arg1: "default", arg2: [42], arg3: "foo" }"#]],
    );

    assert_debug_eq(
        Sut::builder().arg1("arg1".to_owned()).build(),
        expect![[r#"Sut { arg1: "arg1", arg2: [42], arg3: "foo" }"#]],
    );

    assert_debug_eq(
        Sut::builder().maybe_arg1(None::<String>).build(),
        expect![[r#"Sut { arg1: "default", arg2: [42], arg3: "foo" }"#]],
    );

    #[bon]
    impl Sut {
        #[builder]
        fn assoc(
            self,
            #[builder(default = "default".to_owned())] arg1: String,
            #[builder(default = vec![43])] arg2: Vec<u32>,
            #[builder(default = "foo", into)] arg3: String,
        ) -> Self {
            Self {
                arg1: format!("{}+{arg1}", self.arg1),
                arg2: self.arg2.into_iter().chain(arg2).collect(),
                arg3: format!("{}+{arg3}", self.arg3),
            }
        }
    }

    let sut = || Sut::builder().build();

    assert_debug_eq(
        sut().assoc().call(),
        expect![[r#"
            Sut {
                arg1: "default+default",
                arg2: [
                    42,
                    43,
                ],
                arg3: "foo+foo",
            }"#]],
    );

    assert_debug_eq(
        sut().assoc().arg1("arg1".to_owned()).call(),
        expect![[r#"
            Sut {
                arg1: "default+arg1",
                arg2: [
                    42,
                    43,
                ],
                arg3: "foo+foo",
            }"#]],
    );

    assert_debug_eq(
        sut().assoc().maybe_arg1(None).call(),
        expect![[r#"
            Sut {
                arg1: "default+default",
                arg2: [
                    42,
                    43,
                ],
                arg3: "foo+foo",
            }"#]],
    );

    assert_debug_eq(
        sut().assoc().maybe_arg3(None::<String>).call(),
        expect![[r#"
            Sut {
                arg1: "default+default",
                arg2: [
                    42,
                    43,
                ],
                arg3: "foo+foo",
            }"#]],
    );
}

#[test]
fn fn_no_std() {
    #[builder]
    fn sut(#[builder(default)] arg1: u32, #[builder(default = 42)] arg2: u32) -> (u32, u32) {
        (arg1, arg2)
    }

    assert_debug_eq(sut().call(), expect!["(0, 42)"]);
    assert_debug_eq(sut().arg1(12).call(), expect!["(12, 42)"]);
    assert_debug_eq(sut().maybe_arg1(None::<u32>).call(), expect!["(0, 42)"]);
}

#[test]
fn struct_no_std() {
    use bon::bon;

    #[derive(Debug, Builder)]
    struct Sut {
        #[builder(default)]
        arg1: u32,

        #[builder(default = 42)]
        arg2: u32,
    }

    assert_debug_eq(Sut::builder().build(), expect!["Sut { arg1: 0, arg2: 42 }"]);

    assert_debug_eq(
        Sut::builder().arg1(12).build(),
        expect!["Sut { arg1: 12, arg2: 42 }"],
    );

    assert_debug_eq(
        Sut::builder().maybe_arg1(None::<u32>).build(),
        expect!["Sut { arg1: 0, arg2: 42 }"],
    );

    #[bon]
    impl Sut {
        #[builder]
        fn assoc(self, #[builder(default)] arg1: u32, #[builder(default = 43)] arg2: u32) -> Self {
            Self {
                arg1: self.arg1 + arg1,
                arg2: self.arg2 + arg2,
            }
        }
    }

    let sut = || Sut::builder().build();

    assert_debug_eq(sut().assoc().call(), expect!["Sut { arg1: 0, arg2: 85 }"]);
    assert_debug_eq(
        sut().assoc().arg1(12).call(),
        expect!["Sut { arg1: 12, arg2: 85 }"],
    );
    assert_debug_eq(
        sut().assoc().maybe_arg1(None::<u32>).call(),
        expect!["Sut { arg1: 0, arg2: 85 }"],
    );
}

#[test]
fn fn_generic_default() {
    #[builder]
    fn sut(
        #[builder(default)] arg1: impl Clone + Default,
        #[builder(default = <_>::default())] arg2: impl Clone + Default,
    ) {
        drop(arg1);
        drop(arg2);
    }

    sut::<(), ()>().call();
}
