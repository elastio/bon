use crate::prelude::*;

#[test]
fn smoke() {
    #[derive(Debug, Builder)]
    #[builder(derive(Debug, Clone))]
    #[allow(dead_code)]
    struct Sut<T> {
        #[builder(start_fn)]
        x1: u32,

        #[builder(getter(name = x2_with_custom_name))]
        x2: &'static str,

        #[builder(getter(vis = "pub(crate)", doc {
            /// Docs on the getter
        }))]
        x3: u32,

        #[builder(into, getter(name = x5, vis = "pub(crate)", doc {
            /// The name is a lie
        }))]
        x4_but_its_actually_5: &'static str,

        not_a_getter: u32,

        #[builder(getter)]
        generic_option_getter: Option<T>,

        x6: (),

        #[builder(getter, default)]
        x7: u32,
    }

    #[allow(clippy::redundant_clone)]
    let sut = Sut::<()>::builder(0u32).clone();

    let actual = sut.x2("2").x3(3);

    let actual = actual.x4_but_its_actually_5("4");
    let x5 = actual.x5();
    assert_eq!(*x5, "4");

    let actual = actual.not_a_getter(5).x6(());

    let x2 = actual.x2_with_custom_name();
    assert_eq!(*x2, "2");

    let x3 = actual.get_x3();
    assert_eq!(x3, &3);

    let actual = actual.maybe_generic_option_getter(None);

    let gen_opt_get = actual.get_generic_option_getter();
    assert_eq!(gen_opt_get, None);

    let actual = actual.x7(7);
    assert_eq!(actual.get_x7(), Some(&7));

    assert_debug_eq(
        &actual,
        expect![[r#"
            SutBuilder {
                x1: 0,
                x2: "2",
                x3: 3,
                x4_but_its_actually_5: "4",
                not_a_getter: 5,
                x6: (),
                x7: 7,
            }"#]],
    );
}

#[test]
fn copy() {
    #[derive(Debug, Builder)]
    struct Sut {
        #[builder(getter(copy))]
        _x1: u32,
    }

    let sut = Sut::builder().x1(23);
    let x1: u32 = sut.get_x1();
    assert_eq!(x1, 23);
}

#[test]
#[cfg(feature = "std")]
fn deref() {
    use core::ffi::CStr;
    use std::borrow::Cow;
    use std::ffi::{CString, OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[derive(Debug, Builder)]
    struct Sut<'a> {
        #[builder(getter(deref))]
        _vec: Vec<u32>,

        #[builder(getter(deref))]
        _box_: Box<u32>,

        #[builder(getter(deref))]
        _rc: Rc<u32>,

        #[builder(getter(deref))]
        _arc: Arc<u32>,

        #[builder(getter(deref))]
        _string: String,

        #[builder(getter(deref))]
        _c_string: CString,

        #[builder(getter(deref))]
        _os_string: OsString,

        #[builder(getter(deref))]
        _path_buf: PathBuf,

        #[builder(getter(deref))]
        _cow: Cow<'a, str>,
    }

    let builder = Sut::builder()
        .vec(vec![1, 2, 3])
        .box_(Box::new(4))
        .rc(Rc::new(5))
        .arc(Arc::new(6))
        .string("7".to_string())
        .c_string(CString::new("8").unwrap())
        .os_string(OsString::from("9"))
        .path_buf(PathBuf::from("10"))
        .cow(Cow::Borrowed("11"));

    let actual = (
        assert_getter::<&[u32], _>(&builder, SutBuilder::get_vec),
        assert_getter::<&u32, _>(&builder, SutBuilder::get_box_),
        assert_getter::<&u32, _>(&builder, SutBuilder::get_rc),
        assert_getter::<&u32, _>(&builder, SutBuilder::get_arc),
        assert_getter::<&str, _>(&builder, SutBuilder::get_string),
        assert_getter::<&CStr, _>(&builder, SutBuilder::get_c_string),
        assert_getter::<&OsStr, _>(&builder, SutBuilder::get_os_string),
        assert_getter::<&Path, _>(&builder, SutBuilder::get_path_buf),
        assert_getter::<&str, _>(&builder, SutBuilder::get_cow),
    );

    assert_debug_eq(
        actual,
        expect![[r#"([1, 2, 3], 4, 5, 6, "7", "8", "9", "10", "11")"#]],
    );
}

/// Helper function that is better than just `let _: ExpectedType = builder.get_foo();`
/// this notation involves an implicit deref coercion, but we want to assert the exact
/// return type of the getter without any additional implicit conversions.
fn assert_getter<'a, T, B>(builder: &'a B, method: impl FnOnce(&'a B) -> T) -> T {
    method(builder)
}
