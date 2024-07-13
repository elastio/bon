use bon::builder;
use expect_test::expect;

#[test]
fn smoke() {
    /// Docs on struct itself.
    /// Multiline.
    #[builder]
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Sut<'a> {
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
            u32: 42,
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
