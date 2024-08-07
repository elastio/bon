mod expose_positional_fn;

use bon::{bon, builder};
use core::num::NonZeroU32;
#[cfg(feature = "alloc")]
use {
    alloc::borrow::ToOwned, alloc::collections::BTreeSet, alloc::format, alloc::string::String,
    alloc::vec, alloc::vec::Vec,
};

#[cfg(feature = "alloc")]
#[test]
fn smoke() {
    /// Function-level docs
    /// multiline.
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
        arg2: &'_ str,
        arg3: String,
        arg4: u32,

        /// Docs on optional parameter
        arg5: Option<u32>,
        arg6: Option<&str>,
        arg7: Vec<String>,
        arg8: (u32, &[bool]),
    ) -> String {
        drop((arg1, arg2, arg4, arg5, arg6, arg7, arg8));
        arg3
    }

    let actual = sut()
        .arg1(true)
        .arg2("arg2")
        .arg3("arg3")
        .arg4(1)
        .arg7(vec!["arg7".to_owned()])
        .arg8((1, &[true]))
        .call();

    assert_eq!(actual, "arg3");
}

#[cfg(feature = "alloc")]
#[test]
fn default_attr_alloc() {
    #[builder]
    fn sut(
        #[builder(default = "default")] arg3: String,
        #[builder(default = vec![42])] arg4: Vec<u32>,
    ) -> (String, Vec<u32>) {
        (arg3, arg4)
    }

    let actual = sut().call();

    assert_eq!(actual, ("default".to_owned(), vec![42]));
}

#[test]
fn default_attr_no_std() {
    #[builder]
    fn sut(#[builder(default)] arg1: u32, #[builder(default = 42)] arg2: u32) -> (u32, u32) {
        (arg1, arg2)
    }

    let actual = sut().call();

    assert_eq!(actual, (0, 42));
}

#[cfg(feature = "alloc")]
#[test]
fn into_attr_alloc() {
    #[builder]
    fn sut(
        #[builder(into)] set: Option<BTreeSet<u32>>,
        #[builder(into = false)] disabled_into: String,
    ) -> String {
        format!("{set:?}:{disabled_into}")
    }

    let actual = sut()
        .set([32, 43])
        .disabled_into("disabled".to_owned())
        .call();

    assert_eq!(actual, "Some({32, 43}):disabled");
}

#[test]
fn into_attr_no_std() {
    #[builder]
    fn sut(
        #[builder(into)] str_ref: &str,

        /// Some docs
        #[builder(into)]
        u32: u32,
    ) -> (&str, u32) {
        (str_ref, u32)
    }

    struct IntoStrRef<'a>(&'a str);

    impl<'a> From<IntoStrRef<'a>> for &'a str {
        fn from(val: IntoStrRef<'a>) -> Self {
            val.0
        }
    }

    let actual = sut()
        .str_ref(IntoStrRef("vinyl-scratch"))
        .u32(NonZeroU32::new(32).unwrap())
        .call();

    assert_eq!(actual, ("vinyl-scratch", 32));
}

#[cfg(feature = "alloc")]
#[test]
fn into_string() {
    #[builder]
    fn sut(arg1: String, arg2: Option<String>) -> String {
        format!("{arg1}:{arg2:?}")
    }

    let actual = sut().arg1("blackjack").arg2("bruh").call();
    assert_eq!(actual, "blackjack:Some(\"bruh\")");

    let actual = sut().arg1("blackjack").maybe_arg2(Some("bruh2")).call();
    assert_eq!(actual, "blackjack:Some(\"bruh2\")");
}

#[test]
fn leading_underscore_is_stripped() {
    #[builder]
    fn sut(#[builder(default)] _arg1: bool, _arg2: Option<()>) {}

    sut().arg1(true).call();
    sut().arg2(()).call();
    sut().maybe_arg2(Some(())).call();
}

#[test]
fn lifetime_elision() {
    #[builder]
    fn sut(arg: &str, _arg2: usize) -> (&str, &str, [&str; 1]) {
        (arg, arg, [arg])
    }

    let actual = sut().arg("blackjack").arg2(32).call();
    assert_eq!(actual, ("blackjack", "blackjack", ["blackjack"]));
}

#[cfg(feature = "std")]
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
#[allow(unsafe_code)]
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
    #[allow(dropping_copy_types)]
    fn sut(
        /// Some documentation
        iterable: impl IntoIterator<Item = impl Into<u32>>,
        multi_bounds: impl Send + Copy,
    ) {
        drop(iterable.into_iter().map(Into::into));
        drop(multi_bounds);
        drop(multi_bounds);
    }

    sut().iterable([1_u16, 2, 3]).multi_bounds("multi").call();
}

#[test]
fn constructor() {
    struct Counter {
        val: u32,
    }

    #[bon]
    impl Counter {
        #[builder(expose_positional_fn = new)]
        fn new(initial: Option<u32>) -> Self {
            Self {
                val: initial.unwrap_or_default(),
            }
        }
    }

    let counter = Counter::builder().initial(3).build();

    assert_eq!(counter.val, 3);

    let counter = Counter::new(Some(32));

    assert_eq!(counter.val, 32);
}

#[test]
fn receiver() {
    #[derive(Clone)]
    struct Counter {
        val: u32,
    }

    #[bon]
    impl Counter {
        /// Docs on the method.
        /// Multiline
        #[builder]
        fn increment(&self, #[builder(default)] disabled: bool) -> Self {
            if disabled {
                return self.clone();
            }
            Self { val: self.val + 1 }
        }
    }

    let counter = Counter { val: 0 };
    let counter = counter.increment().call();

    assert_eq!(counter.val, 1);
}

#[test]
fn receiver_with_lifetimes() {
    struct Sut<'a, 'b> {
        a: &'a str,
        b: &'b str,
    }

    #[bon]
    impl Sut<'_, '_> {
        #[builder]
        fn method(&self, c: &str) -> usize {
            let Self { a, b } = self;

            a.len() + b.len() + c.len()
        }
    }

    let actual = Sut { a: "a", b: "b" }.method().c("c").call();
    assert_eq!(actual, 3);
}

#[test]
fn self_in_a_bunch_of_places() {
    struct Sut;

    #[bon]
    impl Sut
    where
        Self: Sized + 'static,
    {
        #[builder]
        fn method(&self, me: Option<Self>) -> impl Iterator<Item = Self>
        where
            Self: Sized,
        {
            me.into_iter()
        }
    }

    assert_eq!(Sut.method().me(Sut).call().count(), 1);
}

#[test]
fn receiver_is_non_default() {
    struct Sut {
        val: u32,
    }

    #[bon]
    impl Sut {
        #[builder]
        fn method(self: &Sut) -> u32 {
            self.val
        }
    }

    let sut = Sut { val: 42 };

    assert_eq!(sut.method().call(), 42);
}

#[test]
fn impl_block_ty_contains_a_reference() {
    struct Sut<T>(T);

    #[bon]
    impl<T> Sut<&'_ T> {
        #[builder]
        fn get(&self) -> &T {
            self.0
        }
    }

    assert_eq!(Sut(&42).get().call(), &42);
}

#[test]
fn impl_block_with_self_in_const_generics() {
    #[derive(Default)]
    struct Sut<const N: usize>;

    impl<const N: usize> Sut<N> {
        const fn val(&self) -> usize {
            42
        }
    }

    #[bon]
    impl Sut<{ Sut::<3>.val() }>
    where
        Self:,
    {
        #[builder]
        fn method(self) -> usize {
            self.val()
        }
    }

    assert_eq!(Sut::<42>.method().call(), 42);
}

#[test]
fn generics_with_lifetimes() {
    #[builder]
    fn sut<T>(arg: &&&&&T) {
        let _ = arg;
    }

    sut().arg(&&&&&&&&&&42).call();
}

#[test]
fn const_function() {
    #[builder]
    const fn foo(_arg: u32) {}

    foo().arg(42).call();
}

// This is based on the issue https://github.com/elastio/bon/issues/8
#[test]
#[allow(non_camel_case_types)]
fn raw_identifiers() {
    #[builder]
    fn r#type(r#type: u32, #[builder(name = r#while)] other: u32) {
        let _ = (r#type, other);
    }

    r#type().r#type(42).r#while(100).call();

    #[builder(builder_type = r#type)]
    fn sut() {}

    let _: r#type = sut();
}

// This is based on the issue https://github.com/elastio/bon/issues/16
#[test]
fn self_only_generic_param() {
    struct Sut<'a, 'b: 'a, T> {
        bar: Option<T>,
        str: &'a str,
        other_ref: &'b (),
    }

    #[bon]
    impl<T> Sut<'_, '_, T> {
        #[builder]
        fn new() -> Self {
            Self {
                bar: None,
                str: "littlepip",
                other_ref: &(),
            }
        }
    }

    // Make sure `new` method is hidden
    Sut::<core::convert::Infallible>::__orig_new();

    // Make sure the builder type name matches the type of builder when
    // `#[builder]` is placed on top of a struct
    let _: SutBuilder<'_, '_, core::convert::Infallible> = Sut::builder();

    let actual = Sut::<core::convert::Infallible>::builder().build();

    assert!(actual.bar.is_none());
    assert_eq!(actual.str, "littlepip");
    let () = actual.other_ref;
}

#[test]
fn mut_fn_params() {
    #[builder]
    fn sut(mut arg1: u32, mut arg2: u32) -> (u32, u32) {
        arg1 += 1;
        arg2 += 2;

        (arg1, arg2)
    }

    let actual = sut().arg1(1).arg2(2).call();
    assert_eq!(actual, (2, 4));
}

// This is based on the issue https://github.com/elastio/bon/issues/12
#[test]
fn types_not_implementing_default() {
    struct DoesNotImplementDefault;

    #[builder]
    fn test(_value: Option<DoesNotImplementDefault>) {}

    test().call();
}
