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

pub use bon_macros::__prettier_type_aliases_docs;
pub use rustversion;

pub(crate) mod sealed {
    // The purpose of the `Sealed` trait **is** to be unnameable from outside the crate.
    #[allow(unnameable_types)]
    pub trait Sealed: Sized {}

    impl<Name> Sealed for super::Unset<Name> {}
    impl<Name> Sealed for super::Set<Name> {}
}

use sealed::Sealed;

/// Used to implement the `alloc` feature.
#[cfg(feature = "alloc")]
pub extern crate alloc;

#[derive(Debug)]
pub struct Unset<Name>(Name);

#[derive(Debug)]
pub struct Set<Name>(Name);

#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "expected the type state for the member `{Name}`, but found `{Self}`",
        label = "expected the type state for the member `{Name}`, but found `{Self}`",
    )
)]
pub trait MemberState<Name>: Sealed {}

impl<Name> MemberState<Name> for Unset<Name> {}
impl<Name> MemberState<Name> for Set<Name> {}

#[doc = r" Marker trait that indicates that the member is set, i.e. at least"]
#[doc = r" one of its setters was called."]
#[rustversion::attr(since(1.78.0),diagnostic::on_unimplemented(message = "the member `{Self}` was not set, but this method requires it to be set",label = "the member `{Self}` was not set, but this method requires it to be set"))]
pub trait IsSet: Sealed {}

#[doc(hidden)]
impl<Name> IsSet for Set<Name> {}

#[doc = r" Marker trait implemented by members that are not set."]
#[rustversion::attr(since(1.78.0),diagnostic::on_unimplemented(message = "the member `{Self}` was already set, but this method requires it to be unset",label = "the member `{Self}` was already set, but this method requires it to be unset"))]
pub trait IsUnset: Sealed {}
#[doc(hidden)]
impl<Name> IsUnset for Unset<Name> {}
