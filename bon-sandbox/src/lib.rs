#![doc = include_str!("../README.md")]
#![allow(missing_debug_implementations, missing_docs, dead_code)]

pub mod attr_default;
pub mod attr_getter;
pub mod attr_setters_doc_default_skip;
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
