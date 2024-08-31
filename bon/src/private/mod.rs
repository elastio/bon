/// Used for providing better IDE hints (completions and syntax highlighting).
pub mod ide;

/// Used to implement the `alloc` feature.
#[cfg(feature = "alloc")]
pub extern crate alloc;

#[derive(Debug)]
pub struct Unset;

impl<T> From<Unset> for Set<Option<T>> {
    #[inline(always)]
    fn from(_: Unset) -> Set<Option<T>> {
        Set(None)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Set<T>(pub T);
