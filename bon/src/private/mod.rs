#![allow(
    // We place `#[inline(always)]` only on very small methods where we'd event want
    // a guarantee of them being inlined.
    clippy::inline_always,

    // Marking every potential function as `const` is a bit too much.
    // Especially, this doesn't play well with our MSRV. Trait bounds
    // aren't allowed on const functions in older Rust versions.
    clippy::missing_const_for_fn,

    // We use `deprecated` as a sign to the user that they shouldn't use
    // the type as it's an internal implementation detail.
    deprecated,
)]

/// Used to trigger deprecation warnings from the macros.
pub mod deprecations;

/// Used for providing better IDE hints (completions and syntax highlighting).
pub mod ide;

pub mod derives;

mod cfg_eval;
mod member_cell;

pub(crate) mod sealed {
    // The purpose of the `Sealed` trait **is** to be unnameable from outside the crate.
    #[allow(unnameable_types)]
    pub trait Sealed: Sized {}

    impl<T> Sealed for super::Unset<T> {}
    impl<T> Sealed for super::Set<T> {}
}

pub use member_cell::*;

use core::fmt;
use sealed::Sealed;

/// Used to implement the `alloc` feature.
#[cfg(feature = "alloc")]
pub extern crate alloc;

pub fn assert_clone<T: Clone>() {}
pub fn assert_debug<T: ?Sized + fmt::Debug>() {}

#[doc(hidden)]
#[deprecated = "this type is an implementation detail and should not be used directly; \
    if you found yourself needing it, then you are probably doing something wrong; \
    feel free to open an issue/discussion in our GitHub repository \
    (https://github.com/elastio/bon) or ask for help in our Discord server \
    (https://discord.gg/QcBYSamw4c)"]
#[derive(Debug)]
pub struct Unset<T>(T);

impl<T> crate::IsUnset for Unset<T> {
    // type MemberName =;
}

#[doc(hidden)]
#[deprecated = "this type is an implementation detail and should not be used directly; \
    if you found yourself needing it, then you are probably doing something wrong; \
    feel free to open an issue/discussion in our GitHub repository \
    (https://github.com/elastio/bon) or ask for help in our Discord server \
    (https://discord.gg/QcBYSamw4c)"]
#[derive(Debug)]
pub struct Set<T>(T);

impl<T> crate::IsSet for Set<T> {}

pub trait MemberState: Sealed {
    fn is_set() -> bool;
}

impl<T> MemberState for Unset<T> {
    #[inline(always)]
    fn is_set() -> bool {
        false
    }
}

impl<T> MemberState for Set<T> {
    #[inline(always)]
    fn is_set() -> bool {
        true
    }
}
