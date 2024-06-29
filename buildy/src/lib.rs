pub use buildy_macros::*;
use std::mem::MaybeUninit;

#[repr(transparent)]
pub struct Required<T> {
    /// `MaybeUninit` is used to make the memory layout of `Required<T>` be equal
    /// to `T` such that the compiler may optimize away moving data between
    /// `Required<T>` and `Specified<T>`.
    inner: MaybeUninit<T>,
}

impl<T> Required<T> {
    pub fn new() -> Self {
        Self {
            inner: MaybeUninit::uninit(),
        }
    }
}

impl<T> Default for Required<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(transparent)]
pub struct Optional<T>(Option<T>);

impl<T> Optional<T> {
    pub fn new() -> Self {
        Self(None)
    }
}

impl<T> Default for Optional<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<Optional<T>> for Set<Option<T>> {
    fn from(optional: Optional<T>) -> Self {
        Set::new(optional.0)
    }
}

#[repr(transparent)]
pub struct Set<T>(T);

impl<T> Set<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}
