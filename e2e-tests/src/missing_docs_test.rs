//! This is based on the issue <https://github.com/elastio/bon/issues/38>
#![warn(missing_docs)]

use bon::{bon, builder, Builder};

/// [`MyStruct`] docs
pub struct MyStruct;

#[bon::bon]
impl MyStruct {
    /// [`MyStruct::builder()`] docs
    #[builder]
    pub fn new() -> Self {
        eprintln!("Non-const");
        Self
    }
}

/// [`function`] docs
#[builder]
pub fn function(
    // Docs on setters for function parameters are autogenerated
    // So missing docs here shouldn't be reported
    _arg1: u32,
    _arg2: bool,
) {
    eprintln!("Non-const");
}

/// [`Struct`] docs
#[derive(Builder)]
pub struct Struct {
    // Docs on setters for struct fields are autogenerated
    // So missing docs here shouldn't be reported
    _field1: String,
    _field2: usize,
}

#[bon]
impl Struct {
    /// [`Struct::method()`] docs
    #[builder]
    pub fn method(&self, _arg1: u32, _arg2: bool) {
        eprintln!("Non-const");
        let _ = self;
    }
}

/// [`GenericStruct`] docs
#[derive(bon::Builder)]
pub struct GenericStruct<T> {
    // Docs on setters for struct fields are autogenerated
    // So missing docs here shouldn't be reported
    _field: T,
}
