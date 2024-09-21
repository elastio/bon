use super::MemberState;
use crate::{IsSet, IsUnset};
use core::fmt;
use core::marker::PhantomData;
use core::mem::MaybeUninit;

/// This is an optimized version of the [`Option`] type that encodes the state
/// of [`Some`] or [`None`] at compile time, and thus it's a completely zero-cost
/// transparent wrapper over the inner value of type `T`.
///
/// The `State` generic type determines whether the value was set or not. It can
/// either implement the [`IsSet`] trait or the [`IsUnset`] trait but never both (!).
///
/// This requirement that [`IsSet`] and [`IsUnset`] are mutually exclusive is
/// enforced by the type system and also the fact that these traits are sealed,
/// so nothing outside of this module can implement them.
///
/// The [`MemberState`] trait bound is required because it's needed in the [`Drop`]
/// implementation which does a conditional [`drop`] based on the state of the value
/// (i.e., only when it [`IsSet`]). This is basically a workaround for the lack of
/// specialization in Rust, and that the compiler still conservatively assumes that
/// [`IsSet`] and [`IsUnset`] trait implementations can possible overlap even though
/// they are sealed.
#[doc(hidden)]
#[must_use]
pub struct MemberCell<State: MemberState, T> {
    /// The [`PhantomData`] uses an `fn()` pointer to signify that this type
    /// doesn't hold an instance of `State`.
    state: PhantomData<fn() -> State>,

    value: MaybeUninit<T>,
}

impl<State: MemberState, T> Drop for MemberCell<State, T> {
    fn drop(&mut self) {
        if State::is_set() {
            // SAFETY: this is safe. The `value` is guaranteed to be initialized
            // because `State::is_set()` returns `true` only when it implements
            // the `IsSet` trait. The `IsSet` trait is sealed and implemented
            // only for the `Set` type, and there is only one way to create
            // a `MaybeCell` for `IsSet` state - via the `MaybeCell::new`
            // constructor that initializes the `value` field.
            //
            // Also the `MaybeCell::into_inner` method runs `mem::forget` on the
            // `MaybeCell` instance once it consumed the inner value, so the
            // `drop` method won't be invoked in that case.
            #[allow(unsafe_code)]
            unsafe {
                // MSRV: we can't use `MaybeUninit::assume_init_drop` here because
                // it is only available since Rust 1.60.0 (or MSRV is 1.59.0)
                core::ptr::drop_in_place(self.value.as_mut_ptr());
            }
        }
    }
}

impl<State: IsUnset + MemberState, T> MemberCell<State, T> {
    /// Creates a new [`MemberCell`] with an uninitialized value. This is only
    /// possible when the `State` implements the [`IsUnset`] trait.
    #[inline(always)]
    pub fn uninit() -> Self {
        Self {
            state: PhantomData,
            value: MaybeUninit::uninit(),
        }
    }
}

impl<State: IsSet + MemberState, T> MemberCell<State, T> {
    /// Creates a new [`MemberCell`] initialized with the specified value. This is
    /// only possible when the `State` implements the [`IsSet`] trait.
    #[inline(always)]
    pub fn new(value: T) -> Self {
        Self {
            state: PhantomData,
            value: MaybeUninit::new(value),
        }
    }

    /// Returns a reference to the value if it's set, otherwise `None`.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        // SAFETY: this is safe. The `value` is guaranteed to be initialized
        // by the `MemberCell::new` constructor. There is no other way to
        // create a `MemberCell` where `State: IsSet`. The trait implementation
        // if `IsSet` and `IsUnset` are guaranteed to be mutually exclusive.
        // They are sealed and implemented for Set/Unset types respectively.
        #[allow(unsafe_code)]
        unsafe {
            // MSRV: we can't use `MaybeUninit::assume_init_read` here because
            // it is only available since Rust 1.60.0 (or MSRV is 1.59.0)
            let value = self.value.as_ptr().read();

            // SAFETY: Make sure `drop` doesn't run to avoid double drop
            // now that we have the `value` moved out of the `MaybeUninit`.
            core::mem::forget(self);

            value
        }
    }
}

impl<T, State: MemberState> MemberCell<State, T> {
    #[inline(always)]
    fn try_get(&self) -> Option<&T> {
        if State::is_set() {
            // SAFETY: this is safe. The `value` is guaranteed to be initialized
            // by the `MemberCell::new` constructor. There is no other way to
            // create a `MemberCell` where `State: IsSet`. The trait implementation
            // if `IsSet` and `IsUnset` are guaranteed to be mutually exclusive.
            // They are sealed and implemented for Set/Unset types respectively.
            #[allow(unsafe_code)]
            unsafe {
                return Some(self.value.assume_init_ref());
            }
        }

        None
    }
}

impl<State, T> Clone for MemberCell<State, T>
where
    State: MemberState,
    T: Clone,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        // `map_or_else` reads works, and writing a raw `if let` here may make
        // it easier for the compiler to optimize this code out.
        #[allow(clippy::option_if_let_else)]
        if let Some(value) = self.try_get() {
            Self {
                state: PhantomData,
                value: MaybeUninit::new(value.clone()),
            }
        } else {
            Self {
                state: PhantomData,
                value: MaybeUninit::uninit(),
            }
        }
    }
}

impl<State, T> fmt::Debug for MemberCell<State, T>
where
    State: MemberState,
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.try_get() {
            fmt::Debug::fmt(value, f)
        } else {
            f.write_str("Unset")
        }
    }
}
