//! Tests that make sure `clippy::missing_docs_in_private_items` isn't triggered.
//! This was created in response to [this issue](https://github.com/elastio/bon/issues/388).
#![warn(clippy::missing_docs_in_private_items)]

use bon::{bon, builder, Builder};

/// Docs on `ExampleStruct`
#[derive(Builder)]
struct ExampleStruct {
    /// Docs on `start_fn` member
    #[builder(start_fn)]
    start_fn: u32,

    /// Docs on `finish_fn` member
    #[builder(finish_fn)]
    finish_fn: u32,

    /// Docs on regular member
    regular: u32,
}

#[bon]
impl ExampleStruct {
    #[builder]
    const fn selfless_fn(
        /// Docs on `start_fn` member
        #[builder(start_fn)]
        start_fn: u32,

        /// Docs on `finish_fn` member
        #[builder(finish_fn)]
        finish_fn: u32,

        /// Docs on regular member
        regular: u32,
    ) {
        let _ = start_fn;
        let _ = finish_fn;
        let _ = regular;
    }

    #[builder]
    const fn selfful_fn(
        &self,

        /// Docs on `start_fn` member
        #[builder(start_fn)]
        start_fn: u32,

        /// Docs on `finish_fn` member
        #[builder(finish_fn)]
        finish_fn: u32,

        /// Docs on regular member
        regular: u32,
    ) {
        let _ = self;
        let _ = start_fn;
        let _ = finish_fn;
        let _ = regular;
    }
}

/// Docs on `example` function
#[builder]
const fn example_function(
    /// Docs on `start_fn` member
    #[builder(start_fn)]
    start_fn: u32,

    /// Docs on `finish_fn` member
    #[builder(finish_fn)]
    finish_fn: u32,

    /// Docs on regular member
    regular: u32,
) {
    let _ = start_fn;
    let _ = finish_fn;
    let _ = regular;
}
