//! Not a real crate! It's just a showcase of examples used by `bon`'s documentation
//! to demonstrate the rustdoc output for code generated by builder macros.
//!
//! Don't use this crate, it doesn't follow semver at all and serves no other puprose
//! other than linking to its docs as an example!
#![allow(missing_debug_implementations, missing_docs, dead_code)]

pub mod attr_default;
pub mod attr_with;
pub mod docs_comparison;
pub mod functions;
pub mod macro_rules_wrapper_test;
pub mod missing_docs_test;
pub mod overrides;
pub mod private_builder;
pub mod state_mod;

mod reexports;

pub use reexports::{UnexportedBuilder, UnexportedStateMod, UnexportedStateModBuilder};
