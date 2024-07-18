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


#[test]
fn default_attr() {
    #[builder]
    struct User {
        #[builder(default)] // [!code highlight]
        level: u32,

        // The default value expression of type `&'static str` is // [!code highlight]
        // automatically converted to `String` here via `Into`.   // [!code highlight]
        #[builder(default = "anon")]                              // [!code highlight]
        name: String,

        #[builder(default = bon::vec!["read"])]
        permissions: Vec<String>,
    }

    let user = User::builder().build();

    assert_eq!(user.name, "anon");

    // `<u32 as Default>::default()` is zero
    assert_eq!(user.level, 0);

    assert_eq!(user.permissions, ["read"]);
}
