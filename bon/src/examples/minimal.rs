//! Minimal example of the generated builder and its typestate API.
//!
//! This documentation was generated as a showcase for the [Builder Extensions]
//! guide
//!
//! [Builder Extensions]: https://bon-rs.com/guide/builder-extensions#builder-type-signature

/// Example struct with the `#[derive(Builder)]` annotation.
#[derive(crate::Builder)]
#[builder(crate = crate, state_mod(vis = "pub"))]
pub struct Example {
    /// Example required member
    x1: u32,
    x2: u32,
}
