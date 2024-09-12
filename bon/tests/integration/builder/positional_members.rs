use crate::prelude::*;

struct IntoStrRef<'a>(&'a str);

impl<'a> From<IntoStrRef<'a>> for &'a str {
    fn from(val: IntoStrRef<'a>) -> Self {
        val.0
    }
}

struct IntoChar(char);

impl From<IntoChar> for char {
    fn from(val: IntoChar) -> Self {
        val.0
    }
}

#[test]
fn smoke_struct() {
    #[derive(Debug, Builder)]
    #[allow(dead_code)]
    struct Sut {
        #[builder(start_fn)]
        starter_1: bool,

        #[builder(start_fn, into)]
        starter_2: char,

        #[builder(start_fn, into)]
        starter_3: Option<&'static str>,

        #[builder(finish_fn)]
        finisher_1: &'static str,

        #[builder(finish_fn, into)]
        finisher_2: &'static str,

        named: u32,
    }

    assert_debug_eq(
        Sut::builder(true, IntoChar('c'), None)
            .named(99)
            .build("1", IntoStrRef("2")),
        expect![[r#"
            Sut {
                starter_1: true,
                starter_2: 'c',
                starter_3: None,
                finisher_1: "1",
                finisher_2: "2",
                named: 99,
            }"#]],
    );

    let _ = Sut::builder(true, 'c', "str");
}

#[test]
fn smoke_fn() {
    #[builder]
    fn sut(
        #[builder(start_fn)] starter_1: bool,
        #[builder(start_fn, into)] starter_2: char,
        #[builder(start_fn, into)] starter_3: Option<&'static str>,
        #[builder(finish_fn)] finisher_1: &'static str,
        #[builder(finish_fn, into)] finisher_2: &'static str,
        named: u32,
    ) -> (
        bool,
        char,
        Option<&'static str>,
        u32,
        &'static str,
        &'static str,
    ) {
        (
            starter_1, starter_2, starter_3, named, finisher_1, finisher_2,
        )
    }

    assert_debug_eq(
        sut(true, IntoChar('c'), None)
            .named(99)
            .call("1", IntoStrRef("2")),
        expect![[r#"(true, 'c', None, 99, "1", "2")"#]],
    );

    let _ = sut(true, 'c', "str");
}

#[test]
fn smoke_impl_block() {
    struct Sut;

    #[bon]
    impl Sut {
        #[builder]
        fn sut(
            #[builder(start_fn)] starter_1: bool,
            #[builder(start_fn, into)] starter_2: char,
            #[builder(start_fn, into)] starter_3: Option<&'static str>,
            #[builder(finish_fn)] finisher_1: &'static str,
            #[builder(finish_fn, into)] finisher_2: &'static str,
            named: u32,
        ) -> (
            bool,
            char,
            Option<&'static str>,
            u32,
            &'static str,
            &'static str,
        ) {
            (
                starter_1, starter_2, starter_3, named, finisher_1, finisher_2,
            )
        }

        #[builder]
        fn with_self(
            &self,
            #[builder(start_fn)] starter_1: bool,
            #[builder(finish_fn)] finisher_1: &'static str,
            named: u32,
        ) -> (bool, u32, &'static str) {
            let _ = self;
            (starter_1, named, finisher_1)
        }
    }

    assert_debug_eq(
        Sut::sut(true, IntoChar('c'), None)
            .named(99)
            .call("1", IntoStrRef("2")),
        expect![[r#"(true, 'c', None, 99, "1", "2")"#]],
    );

    let _ = Sut::sut(true, 'c', "str");

    assert_debug_eq(
        Sut.with_self(true).named(99).call("1"),
        expect![[r#"(true, 99, "1")"#]],
    );
}
