//! This module contains examples of `rustdoc` output for builder macros from different
//! crates applied to different kinds of syntax.
//!
//! The module hierarchy here is `syntax` -> `builder_crate`.
//!
//! All builders were configured to produce roughly similar builder API.
//!
//! The notable exceptions are:
//!
//! - `buildstructor` doesn't support `#[builder(default)]` and `#[builder(into)]`-like annotations;
//! - `buildstructor` doesn't support doc comments on function arguments;
//! - `derive_builder` doesn't support typestate-based builders;
#![expect(
    dead_code,
    unused_variables,
    clippy::needless_pass_by_value,
    clippy::unused_self
)]

/// Examples docs generated with builder macros applied to functions
pub mod functions;

/// Examples docs generated with builder macros applied to structs
pub mod structs;

/// Examples docs generated with builder macros applied to methods
pub mod methods;
