#![allow(non_upper_case_globals, missing_debug_implementations)]

/// Completions for the top-level `builder` attribute.
pub mod builder_top_level {
    use super::*;

    /// Docs for start_fn
    pub const start_fn: Option<Identifier> = None;

    /// Docs for long-form start_fn
    pub mod start_fn {
        use super::*;

        /// Docs for name
        pub const name: Identifier = Identifier;

        /// Docs for vis
        pub const vis: VisibilityString = VisibilityString;
    }

    /// Docs for finish_fn
    pub const finish_fn: Option<Identifier> = None;

    /// Docs for builder_type
    pub const builder_type: Option<Identifier> = None;

    /// Docs for expose_positional_fn
    pub const expose_positional_fn: Option<Identifier> = None;

    /// Docs for long-form expose_positional_fn
    pub mod expose_positional_fn {
        use super::*;

        /// Docs for name
        pub const name: Identifier = Identifier;

        /// Docs for vis
        pub const vis: Option<VisibilityString> = None;
    }

    /// Docs for `on`
    pub mod on {
        use super::*;

        /// Docs for into
        pub const into: Flag = Flag;
    }
}

/// Visibility inside of a string literal. Empty string means private visibility.
///
/// Examples:
///
/// - `""` - the symbol is private (accessible only within the same module)
/// - `"pub"` - the symbol is accessible outside of the crate
/// - `"pub(crate)"` - the symbol is accessible anywhere inside of the crate, but not outside of it
///
/// [Rust reference](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
pub struct VisibilityString;

/// [Rust reference](https://doc.rust-lang.org/reference/identifiers.html)
pub struct Identifier;

/// The presence of this attribute enables the behavior. The attribute has no value.
pub struct Flag;
