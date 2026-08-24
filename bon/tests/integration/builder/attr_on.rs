use crate::prelude::*;

struct IntoBool(bool);

impl From<IntoBool> for bool {
    fn from(value: IntoBool) -> Self {
        value.0
    }
}

struct IntoUnit;

impl From<IntoUnit> for () {
    fn from(IntoUnit: IntoUnit) {}
}

struct IntoStrRef<'a>(&'a str);

impl<'a> From<IntoStrRef<'a>> for &'a str {
    fn from(value: IntoStrRef<'a>) -> Self {
        value.0
    }
}

struct Generic<T>(T);
struct IntoGeneric<T>(T);

impl<T> From<IntoGeneric<T>> for Generic<T> {
    fn from(value: IntoGeneric<T>) -> Self {
        Self(value.0)
    }
}

#[test]
fn match_any() {
    #[builder(on(_, into))]
    fn sut<T>(_arg1: bool, _arg2: Option<()>, _arg3: T) {}

    sut::<&str>()
        .arg1(IntoBool(true))
        .arg2(IntoUnit)
        .arg3(IntoStrRef("foo"))
        .call();
}

#[test]
fn match_str_ref() {
    #[builder(on(&str, into))]
    fn sut(_arg1: bool, _arg2: Option<()>, _arg3: &str) {}

    sut().arg1(true).arg2(()).arg3(IntoStrRef("foo")).call();
}

#[test]
fn match_path() {
    #[builder(on(bool, into))]
    fn sut<T>(_arg1: bool, _arg2: Option<()>, _arg3: T) {}

    sut::<&str>()
        .arg1(IntoBool(true))
        .arg2(())
        .arg3("foo")
        .call();
}

#[test]
fn match_generic() {
    #[builder(on(Generic<_>, into))]
    fn sut<T>(_arg1: bool, _arg2: Option<()>, _arg3: Generic<T>) {}

    sut().arg1(true).arg2(()).arg3(IntoGeneric("foo")).call();
}

#[test]
fn default_match_path() {
    #[derive(Debug, Builder)]
    #[builder(on(u32, default))]
    #[allow(dead_code)]
    struct Sut {
        x: u32,
        y: u32,

        // A member-level default with a value wins over `on(u32, default)`.
        #[builder(default = 42)]
        z: u32,
    }

    assert_debug_eq(Sut::builder().build(), expect!["Sut { x: 0, y: 0, z: 42 }"]);

    assert_debug_eq(
        Sut::builder().x(1).y(2).z(3).build(),
        expect!["Sut { x: 1, y: 2, z: 3 }"],
    );
}

#[cfg(feature = "alloc")]
#[test]
fn default_match_generic() {
    #[derive(Debug, Builder)]
    #[builder(on(Vec<_>, default))]
    #[allow(dead_code)]
    struct Sut {
        // `name` doesn't match `Vec<_>`, so it stays required.
        name: String,
        tags: Vec<String>,
        ids: Vec<u32>,
    }

    assert_debug_eq(
        Sut::builder().name("bon".to_owned()).build(),
        expect![[r#"Sut { name: "bon", tags: [], ids: [] }"#]],
    );

    assert_debug_eq(
        Sut::builder()
            .name("bon".to_owned())
            .tags(vec!["x".to_owned()])
            .ids(vec![7])
            .build(),
        expect![[r#"Sut { name: "bon", tags: ["x"], ids: [7] }"#]],
    );
}

#[cfg(feature = "alloc")]
#[test]
fn default_with_into() {
    #[builder(on(String, into, default))]
    fn sut(name: String, level: u32) -> impl core::fmt::Debug {
        (name, level)
    }

    // `name` matched `String`, so it accepts `impl Into<String>` and defaults
    // to an empty string; `level` didn't match, so it stays required.
    assert_debug_eq(sut().level(1).call(), expect![[r#"("", 1)"#]]);

    assert_debug_eq(
        sut().name("bon").level(2).call(),
        expect![[r#"("bon", 2)"#]],
    );
}

#[test]
fn default_with_required() {
    #[derive(Debug, Builder)]
    #[builder(on(_, required, default))]
    #[allow(dead_code)]
    struct Sut {
        // `required` turns `Option<_>` members into required ones, and it wins
        // over `default`, so this member must be set with an `Option` value.
        x: Option<u32>,

        // Non-`Option` members are required, so `default` applies to them.
        y: u32,
    }

    assert_debug_eq(
        Sut::builder().x(None).build(),
        expect!["Sut { x: None, y: 0 }"],
    );

    assert_debug_eq(
        Sut::builder().x(Some(1)).y(2).build(),
        expect!["Sut { x: Some(1), y: 2 }"],
    );
}
