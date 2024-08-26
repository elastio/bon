use crate::prelude::*;
use expect_test::expect;

#[cfg(feature = "alloc")]
#[test]
fn struct_alloc() {
    use expect_test::expect;

    #[builder]
    #[derive(Debug)]
    struct Sut {
        #[builder(skip = "skip".to_owned())]
        #[allow(dead_code)]
        arg1: String,

        #[builder(skip = vec![42])]
        #[allow(dead_code)]
        arg2: Vec<u32>,
    }

    assert_debug_eq(
        Sut::builder().build(),
        expect![[r#"Sut { arg1: "skip", arg2: [42] }"#]],
    );
}

#[test]
fn struct_no_std() {
    #[builder]
    #[derive(Debug)]
    struct Sut {
        #[builder(skip)]
        #[allow(dead_code)]
        arg1: u32,

        #[builder(skip = 42)]
        #[allow(dead_code)]
        arg2: u32,
    }

    assert_debug_eq(Sut::builder().build(), expect!["Sut { arg1: 0, arg2: 42 }"]);
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
}

#[test]
fn struct_generic_skipped() {
    #[builder]
    struct Sut<A, B>
    where
        A: Clone + Default,
        B: Clone + Default,
    {
        #[builder(skip)]
        #[allow(dead_code)]
        arg1: A,

        #[builder(skip = <_>::default())]
        #[allow(dead_code)]
        arg2: B,
    }

    let _: Sut<(), ()> = Sut::<(), ()>::builder().build();
}
