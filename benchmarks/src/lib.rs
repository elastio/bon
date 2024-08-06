#![allow(
    unsafe_code,
    dead_code,
    unreachable_pub,
    dropping_copy_types,
    missing_docs,
    clippy::too_many_arguments,
    clippy::boxed_local,
    clippy::let_and_return,
    rustdoc::missing_crate_level_docs
)]

cfg_if::cfg_if! {
    if #[cfg(feature = "args_3")] {
        pub mod args_3;
        pub use args_3 as bench;
    } else if #[cfg(feature = "args_5")] {
        pub mod args_5;
        pub use args_5 as bench;
    } else if #[cfg(feature = "args_10")] {
        pub mod args_10;
        pub use args_10 as bench;
    } else if #[cfg(feature = "args_10_alloc")] {
        pub mod args_10_alloc;
        pub use args_10_alloc as bench;
    } else if #[cfg(feature = "args_10_structs")] {
        pub mod args_10_structs;
        pub use args_10_structs as bench;
    } else {
        pub mod args_20;
        pub use args_20 as bench;
    }
}

#[inline(never)]
pub fn builder_bench() {
    bench::builder_bench();
}

#[inline(never)]
pub fn regular_bench() {
    bench::regular_bench();
}
