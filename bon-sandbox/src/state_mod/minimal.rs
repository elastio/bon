//! Minimal example of the generated builder and its typestate API.
//!
//! This documentation was generated as a showcase for the [Builder's Type Signature]
//! guide
//!
//! [Builder's Type Signature]: https://bon-rs.com/guide/typestate-api/builders-type-signature

/// Example struct with the `#[derive(Builder)]` annotation.
#[derive(bon::Builder)]
#[builder(state_mod(vis = "pub"))]
pub struct Example {
    x1: u32,
    x2: u32,
}
