mod attr_default;
mod attr_expose_positional_fn;
mod attr_into;
mod attr_on;
mod attr_skip;
mod builder_derives;
mod cfgs;
mod generics;
mod init_order;
mod lints;
mod many_params;
mod name_conflicts;
mod positional_members;
mod raw_idents;
mod smoke;

/// Tests for the deprecated features that we still support, but that we'll
/// eventually remove in the future in a new major version release.
mod legacy;

use crate::prelude::*;

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
    async fn sut(arg: u32) -> u32 {
        std::future::ready(arg).await
    }

    let actual = sut().arg(42).call().await;
    assert_eq!(actual, 42);
}

#[cfg(feature = "std")]
#[tokio::test]
async fn async_func_with_future_arg() {
    #[builder]
    async fn sut<Fut: std::future::Future + Send>(fut: Fut) -> Fut::Output {
        fut.await
    }

    fn is_send(_val: impl Send + Sync) {}

    let fut = sut().fut(std::future::ready(42)).call();

    is_send(fut);

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
    fn sut(
        /// Some documentation
        iterable: impl IntoIterator<Item = impl Into<u32>>,
        multi_bounds: impl Send + Copy,
    ) {
        drop(iterable.into_iter().map(Into::into));
        let _ = multi_bounds;
        let _ = multi_bounds;
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
            let _ = self;
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
        #[allow(clippy::use_self)]
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
fn const_function() {
    #[builder]
    const fn foo(_arg: u32) {}

    foo().arg(42).call();
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
