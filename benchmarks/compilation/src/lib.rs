#![allow(
    missing_docs,
    missing_debug_implementations,
    dead_code,
    rustdoc::missing_crate_level_docs
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

#[cfg(feature = "structs_100_fields_10")]
pub mod structs_100_fields_10;

#[cfg(feature = "structs_10_fields_50")]
pub mod structs_10_fields_50;
