//! Showcases the docs output with `setters(doc(default(skip)))` attribute.
//!
//! There are two examples of the usage of this attribute at different levels.
//! Follow the `Source` links of every example to see how the attributes are used.
//!
//! Examples:
//! - [`MemberExampleBuilder`] - usage of the attribute on an individual member
//! - [`TopLevelExampleBuilder`] - usage of the attribute on the top level for all members

#[derive(bon::Builder)]
pub struct MemberExample {
    /// Shows the default (no overrides).
    #[builder(default = 42)]
    shown: u32,

    /// Skips the default (overridden via the attribute).
    #[builder(default = 42, setters(doc(default(skip))))]
    hidden: u32,
}

#[derive(bon::Builder)]
#[builder(on(u32, setters(doc(default(skip)))))]
pub struct TopLevelExample {
    /// Doesn't match the `on(u32, ...)` type pattern, and thus shows
    /// the default in the docs.
    /// If you want to select all members, then use the pattern `_`.
    #[builder(default = 42)]
    shown: i32,

    /// Matches the `on(u32, ...)` type pattern, and thus skips the default
    /// in the docs.
    #[builder(default = 42)]
    hidden: u32,
}
