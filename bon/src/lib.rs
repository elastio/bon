#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// We mark all items from the `private` module as deprecated to signal that they are
// implementation details and should not be used directly. Unfortunately, this triggers
// the deprecation warnings within this crate itself everywhere we use them, so we just
// suppress this lint for the entire crate.
#![allow(deprecated)]

#[doc(hidden)]
#[deprecated = "the items from the `bon::private` module are an implementation detail; \
    they should not be used directly; if you found a need for this, then you are probably \
    doing something wrong; feel free to open an issue/discussion in our GitHub repository \
    (https://github.com/elastio/bon) or ask for help in our Discord server \
    (https://discord.gg/QcBYSamw4c)"]
pub mod private;

/// Small utility declarative macros for creating colletions with [`Into`] conversions.
mod collections;

/// Rexport all macros from the proc-macro crate.
pub use bon_macros::*;

// use private::sealed::Sealed;
//
// /// Marker trait that indicates that the member is not set, i.e. none of its setters were called.
// ///
// /// You should use this trait bound, for example, if you want to extend the builder with custom
// /// setters.
// ///
// /// **Example:**
// ///
// /// ```
// /// #[derive(bon::Builder)]
// /// struct Example {
// ///     x: i32,
// ///     y: i32,
// /// }
// ///
// /// // Import the type aliases for transforming the builder's type state
// /// use example_builder::{SetX, SetY};
// ///
// /// // Add method to the builder
// /// impl<State: example_builder::State> ExampleBuilder<State> {
// ///     fn x_doubled(self, value: i32) -> ExampleBuilder<SetX<State>>
// ///     where
// ///         // The code won't compile without this bound
// ///         State::X: bon::IsUnset,
// ///     {
// ///         self.x(value * 2)
// ///     }
// ///
// ///     fn y_doubled(self, value: i32) -> ExampleBuilder<SetY<State>>
// ///     where
// ///         // The code won't compile without this bound
// ///         State::Y: bon::IsUnset,
// ///     {
// ///        self.y(value * 2)
// ///     }
// /// }
// ///
// /// let example = Example::builder()
// ///     .x_doubled(2)
// ///     .y_doubled(3)
// ///     .build();
// ///
// /// assert_eq!(example.x, 4);
// /// assert_eq!(example.y, 6);
// /// ```
// #[rustversion::attr(
//     since(1.78.0),
//     diagnostic::on_unimplemented(
//         message = "the member `{Self}` was already set, but this method requires it to be unset",
//         label = "the member `{Self}` was already set, but this method requires it to be unset",
//     )
// )]
// pub trait IsUnset: Sealed {}

// /// Marker trait that indicates that the member is set, i.e. at least one of its setters was called.
// // TODO: add examples (they would require having custom renames and visibility overrides for default setters)
// #[rustversion::attr(
//     since(1.78.0),
//     diagnostic::on_unimplemented(
//         message = "the member `{Self}` was not set, but this method requires it to be set",
//         label = "the member `{Self}` was not set, but this method requires it to be set",
//     )
// )]
// pub trait IsSet: Sealed {}
