#![allow(
    missing_docs,
    missing_debug_implementations,
    dead_code,
    rustdoc::missing_crate_level_docs,
    clippy::must_use_candidate
)]

cfg_if::cfg_if! {
    if #[cfg(feature = "bon")] {
        use bon::Builder;
    } else if #[cfg(feature = "typed-builder")] {
        use typed_builder::TypedBuilder as Builder;
    } else if #[cfg(feature = "derive_builder")] {
        use derive_builder::Builder;
    }
}

// With all features on, only one of the builder crates is used.
// These imports mark the other crates as used.
#[cfg(feature = "typed-builder")]
use typed_builder as _;

#[cfg(feature = "derive_builder")]
use derive_builder as _;

#[cfg(feature = "structs_100_fields_10")]
pub mod structs_100_fields_10;

#[cfg(feature = "structs_10_fields_50")]
pub mod structs_10_fields_50;
