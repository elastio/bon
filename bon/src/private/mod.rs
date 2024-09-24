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
mod member;

pub(crate) mod sealed {
    // The purpose of the `Sealed` trait **is** to be unnameable from outside the crate.
    #[allow(unnameable_types)]
    pub trait Sealed: Sized {}

    impl<Name> Sealed for super::Unset<Name> {}
    impl<Name> Sealed for super::Set<Name> {}
}

pub use member::*;

use sealed::Sealed;

/// Used to implement the `alloc` feature.
#[cfg(feature = "alloc")]
pub extern crate alloc;

#[derive(Debug)]
pub struct Unset<Name>(Name);

impl<Name> crate::IsUnset for Unset<Name> {}

#[derive(Debug)]
pub struct Set<Name>(Name);

impl<Name> crate::IsSet for Set<Name> {}

/// Allows to statically check if a member is set or not.
/// This is basically a utility to do compile-time downcasts.
pub trait MemberState: Sealed {
    fn is_set() -> bool;
}

impl<Name> MemberState for Unset<Name> {
    #[inline(always)]
    fn is_set() -> bool {
        false
    }
}

impl<Name> MemberState for Set<Name> {
    #[inline(always)]
    fn is_set() -> bool {
        true
    }
}

#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "expected type state for the member `{Name}`, but found `{Self}`",
        label = "expected type state for the member `{Name}`, but found `{Self}`",
    )
)]
pub trait NamedMemberState<Name>: MemberState {}
impl<Name> NamedMemberState<Name> for Set<Name> {}
impl<Name> NamedMemberState<Name> for Unset<Name> {}
