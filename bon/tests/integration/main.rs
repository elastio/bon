use bon::{bon, builder};

#[test]
fn smoke() {
    #[builder]
    fn sut(
        /// ### Documentation
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
        arg8: (u32, &[bool]),
    ) -> String {
        let _ = (arg1, arg2, arg4, arg5, arg6, arg7, arg8);
        arg3
    }

    let actual = sut()
        .arg1(true)
        .arg2("arg2")
        .arg3("arg3".to_string())
        .arg4(1)
        .arg5(Some(1))
        .arg6(Some("arg6"))
        .arg7(vec!["arg7".to_string()])
        .arg8((1, &[true]))
        .call();

    assert_eq!(actual, "arg3");
}

#[test]
fn lifetime_elision() {
    #[builder]
    fn sut(arg: &str, _arg2: usize) -> (&str, &str, [&str; 1]) {
        (arg, arg, [arg])
    }

    let actual = sut().arg("blackjack")._arg2(32).call();
    assert_eq!(actual, ("blackjack", "blackjack", ["blackjack"]));
}

#[tokio::test]
async fn async_func() {
    #[builder]
    async fn sut<Fut: std::future::Future>(fut: Fut) -> Fut::Output {
        fut.await
    }

    let actual = sut().fut(async { 42 }).call().await;
    assert_eq!(actual, 42);
}

#[test]
fn unsafe_func() {
    #[builder]
    unsafe fn sut(arg: bool) {
        let _ = arg;
    }

    let builder = sut().arg(true);

    // Only the call method should be unsafe
    unsafe { builder.call() };
}

#[test]
fn impl_traits() {
    #[builder]
    fn sut(
        /// Some documentation
        iterable: impl IntoIterator<Item = impl Into<u32>> + '_,
        showable: impl std::fmt::Display + std::fmt::Debug + '_,
    ) -> (String, Vec<u32>) {
        let str = format!("{showable} + {showable:#?}");
        let vec = iterable.into_iter().map(Into::into).collect();

        (str, vec)
    }

    let (str, vec) = sut()
        .iterable(vec![1_u32, 2, 3])
        .showable("showable")
        .call();

    assert_eq!(str, "showable + \"showable\"");
    assert_eq!(vec, [1, 2, 3]);
}

#[test]
fn receiver() {
    #[derive(Clone)]
    struct Counter {
        val: u32,
    }

    #[bon::bon]
    impl Counter {
        #[builder]
        fn increment(&self, disabled: bool) -> Self {
            if disabled {
                return self.clone();
            }
            Self { val: self.val + 1 }
        }
    }

    let counter = Counter { val: 0 };

    counter.increment().disabled(false).call();

    assert_eq!(counter.val, 1);
}

#[test]
fn receiver_is_non_default() {
    struct Sut {
        str: String,
    }

    #[bon]
    impl Sut {
        #[builder]
        fn method(self: &Sut) -> &str {
            &self.str
        }
    }

    let sut = Sut {
        str: "blackjack".to_string(),
    };

    assert_eq!(sut.method().call(), "blackjack");
}

#[test]
fn impl_block_ty_contains_a_reference() {
    struct Sut<T>(T);

    #[bon]
    impl<T> Sut<&T> {
        #[builder]
        fn get(&self) -> &T {
            self.0
        }
    }

    assert_eq!(Sut(&42).get().call(), &42);
}

#[test]
fn impl_block_with_self_in_const_generics() {
    struct Sut<const N: usize>;

    impl<const N: usize> Sut<N> {
        const fn val(&self) -> usize {
            42
        }
    }

    #[bon]
    impl Sut<{ Sut::<3>.val() }> {
        #[builder]
        fn method(self) -> usize {
            self.val()
        }
    }

    assert_eq!(Sut::<42>.method().call(), 42);
}
