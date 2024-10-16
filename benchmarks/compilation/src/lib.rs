#![allow(missing_docs, missing_debug_implementations)]

cfg_if::cfg_if! {
    if #[cfg(feature = "bon")] {
        use bon::Builder;
    } else if #[cfg(feature = "typed-builder")] {
        use typed_builder::TypedBuilder as Builder;
    } else if #[cfg(feature = "derive_builder")] {
        use derive_builder::Builder;
    }
}

pub mod structs_100_fields_10;
