#![allow(
    unsafe_code,
    dead_code,
    unreachable_pub,
    dropping_copy_types,
    missing_docs,
    clippy::too_many_arguments,
    clippy::boxed_local,
    clippy::let_and_return
)]

#[cfg(feature = "args_10")]
pub mod args_10;

#[cfg(feature = "args_10")]
use args_10 as bench;

#[cfg(feature = "args_10_inline")]
pub mod args_10_inline;

#[cfg(feature = "args_10_inline")]
use args_10_inline as bench;

#[inline(never)]
pub fn builder_bench() {
    bench::builder_bench();
}

#[inline(never)]
pub fn regular_bench() {
    bench::regular_bench();
}
