// use std::mem::MaybeUninit;

/// [`MaybeUninit`] is used to make the memory layout of this struct be equal
/// to `T` such that the compiler may optimize away moving data between it and
/// [`Set<T>`].
#[derive(Debug)]
struct Unset<T>(std::marker::PhantomData<T>);

impl<T> Default for Unset<T> {
    #[inline(always)]
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

#[derive(Debug)]
pub struct Required<T>(Unset<T>);

impl<T> Default for Required<T> {
    #[inline(always)]
    fn default() -> Self {
        Self(Unset::default())
    }
}

#[derive(Debug)]
pub struct Optional<T>(Unset<Option<T>>);

impl<T> Default for Optional<T> {
    #[inline(always)]
    fn default() -> Self {
        Self(Unset::default())
    }
}

impl<T> IntoSet<Option<T>> for Optional<T> {
    #[inline(always)]
    fn into_set(self) -> Set<Option<T>> {
        Set::new(None)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Set<T>(pub T);

impl<T> Set<T> {
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> IntoSet<T> for Set<T> {
    #[inline(always)]
    fn into_set(self) -> Self {
        self
    }
}

pub trait IntoSet<T> {
    fn into_set(self) -> Set<T>;
}
