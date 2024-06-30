use buildy::builder;

#[test]
fn smoke() {
    #[builder]
    fn sut(
        /// # Documentation
        /// **Docs** for arg1.
        ///
        /// Multiline with `code` *examples* __even__!
        ///
        /// ```
        /// let wow_such_code = true;
        /// println!("Code is so lovely! {wow_such_code}");
        /// ```
        ///
        /// - List item 1
        /// - List item 2
        arg1: bool,

        /// Docs for arg2
        arg2: &str,
        arg3: String,
        arg4: u32,
        arg5: Option<u32>,
        arg6: Option<&str>,
        arg7: Vec<String>,
    ) -> String {
        let _ = (arg1, arg2, arg4, arg5, arg6, arg7);

        arg3.sd
    }

    let returned = sut()
        .arg1(true)
        .arg2("arg2")
        .arg3("arg3".to_string())
        .arg4(1)
        .arg5(Some(1))
        .arg6(Some("arg6"))
        .arg7(vec!["arg7".to_string()])
        .call();

    assert_eq!(returned, "asd");
}

// #[test]
// fn nested_items_in_fn() {
//     struct Foo;
//     struct Bar;

//     mod imp {
//         use super::*;

//         struct Builder {
//             bar: Bar,
//         }
//     }

//     fn sut(bar: Bar) {
//         impl Foo {
//             fn bar() {}
//         }
//     }

//     Foo::bar();
// }
