#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_local_definitions)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod builder_on_fn;
mod builder_on_struct;

mod ui;
