/// Used to implement the `alloc` feature.
#[cfg(feature = "alloc")]
pub extern crate alloc;

#[derive(Debug)]
pub struct Required<T>(pub core::marker::PhantomData<T>);

#[derive(Debug)]
pub struct Optional<T>(pub core::marker::PhantomData<Option<T>>);

impl<T> From<Optional<T>> for Set<Option<T>> {
    #[inline(always)]
    fn from(_: Optional<T>) -> Set<Option<T>> {
        const { Set(None) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Set<T>(pub T);
