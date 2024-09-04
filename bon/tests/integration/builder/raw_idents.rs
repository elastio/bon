//! This is based on the issue <https://github.com/elastio/bon/issues/8>
use crate::prelude::*;

#[test]
#[allow(non_camel_case_types)]
fn struct_case() {
    #[derive(Builder)]
    struct r#Type {
        r#type: u32,

        #[builder(name = r#while)]
        other: u32,
    }

    let actual = r#Type::builder().r#type(42).r#while(100).build();

    assert_eq!(actual.r#type, 42);
    assert_eq!(actual.other, 100);

    #[derive(Builder)]
    #[builder(builder_type = r#type)]
    #[allow(clippy::items_after_statements)]
    struct Sut {}

    let _: r#type = Sut::builder();
}

#[test]
#[allow(non_camel_case_types)]
fn fn_case() {
    #[builder]
    fn r#type(r#type: u32, #[builder(name = r#while)] other: u32) {
        let _ = (r#type, other);
    }

    r#type().r#type(42).r#while(100).call();

    #[builder(builder_type = r#type)]
    fn sut() {}

    let _: r#type = sut();
}
