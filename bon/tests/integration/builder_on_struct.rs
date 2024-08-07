use bon::builder;
use expect_test::expect;

#[test]
fn smoke() {
    /// Docs on struct itself.
    /// Multiline.
    #[builder]
    #[allow(dead_code)]
    #[derive(Debug)]
    pub(crate) struct Sut<'a> {
        /// Docs on bool field.
        /// Multiline.
        bool: bool,

        str_ref: &'a str,

        #[builder(default)]
        u32: u32,

        /// Docs on option field.
        /// Multiline.
        option_u32: Option<u32>,

        option_str_ref: Option<&'a str>,
        tuple: (u32, &'a [bool]),
    }

    let actual = Sut::builder()
        .bool(true)
        .str_ref("str_ref")
        .maybe_option_u32(Some(42))
        .option_str_ref("value")
        .tuple((42, &[true, false]))
        .build();

    let expected = expect![[r#"
        Sut {
            bool: true,
            str_ref: "str_ref",
            u32: 0,
            option_u32: Some(
                42,
            ),
            option_str_ref: Some(
                "value",
            ),
            tuple: (
                42,
                [
                    true,
                    false,
                ],
            ),
        }
    "#]];

    expected.assert_debug_eq(&actual);
}

// This is based on the issue https://github.com/elastio/bon/issues/8
#[test]
#[allow(non_camel_case_types)]
fn raw_identifiers() {
    #[builder]
    struct r#Type {
        r#type: u32,

        #[builder(name = r#while)]
        other: u32,
    }

    let actual = r#Type::builder().r#type(42).r#while(100).build();

    assert_eq!(actual.r#type, 42);
    assert_eq!(actual.other, 100);

    #[builder(builder_type = r#type)]
    struct Sut {}

    let _: r#type = Sut::builder();
}
