/// Used for providing better IDE hints (completions and syntax highlighting).
pub mod ide;

/// Used to implement the `alloc` feature.
#[cfg(feature = "alloc")]
pub extern crate alloc;

/// Marker trait to denote the state of the member that is not set yet.
#[diagnostic::on_unimplemented(
    message = "can't set the same member twice",
    label = "this member was already set"
)]
pub trait IsUnset {}

/// The sole implementation of the [`IsUnset`] trait.
#[derive(Debug)]
pub struct Unset;

impl IsUnset for Unset {}

/// A trait used to transition optional members to the [`Set`] state.
/// It also provides a better error message when the member is not set.
/// The `Member` generic parameter isn't used by the trait implementation,
/// it's used only as a label with the name of the member to specify which one
/// was not set.
#[diagnostic::on_unimplemented(
    message = "can't finish building yet",
    label = "the member `{Member}` was not set",
)]
pub trait IntoSet<T, Member> {
    fn into_set(self) -> Set<T>;
}

impl<T, Member> IntoSet<T, Member> for Set<T> {
    fn into_set(self) -> Set<T> {
        self
    }
}

impl<T, Member> IntoSet<Option<T>, Member> for Unset {
    fn into_set(self) -> Set<Option<T>> {
        Set(None)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Set<T>(pub T);
