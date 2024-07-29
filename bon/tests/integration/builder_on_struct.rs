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

        string: String,

        #[builder(default)]
        u32: u32,

        /// Docs on option field.
        /// Multiline.
        option_u32: Option<u32>,

        option_str_ref: Option<&'a str>,
        vec_string: Vec<String>,
        tuple: (u32, &'a [bool]),
    }

    let actual = Sut::builder()
        .bool(true)
        .str_ref("str_ref")
        .string("string")
        .maybe_option_u32(Some(42))
        .option_str_ref("value")
        .vec_string(vec!["String".to_owned()])
        .tuple((42, &[true, false]))
        .build();

    let expected = expect![[r#"
        Sut {
            bool: true,
            str_ref: "str_ref",
            string: "string",
            u32: 0,
            option_u32: Some(
                42,
            ),
            option_str_ref: Some(
                "value",
            ),
            vec_string: [
                "String",
            ],
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
        r#type: String,

        #[builder(name = r#while)]
        other: String,
    }

    let actual = r#Type::builder().r#type("value").r#while("value2").build();

    assert_eq!(actual.r#type, "value");
    assert_eq!(actual.other, "value2");

    #[builder(builder_type = r#type)]
    struct Sut {}

    let _: r#type = Sut::builder();
}

// This is based on the issue https://github.com/elastio/bon/issues/12
#[test]
fn types_not_implementing_default() {
    struct DoesNotImplementDefault;

    #[builder]
    fn test(test_type: Option<DoesNotImplementDefault>) {}

    test().call();
}
