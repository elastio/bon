#![allow(
    non_upper_case_globals,
    missing_debug_implementations,
    clippy::wildcard_imports
)]

/// Completions for the top-level `builder` attribute.
pub mod builder_top_level {
    use super::*;

    /// See the docs at <https://elastio.github.io/bon/reference/builder#start-fn>
    pub const start_fn: Option<Identifier> = None;

    /// See the docs at <https://elastio.github.io/bon/reference/builder#start-fn>
    pub mod start_fn {
        use super::*;

        /// See the docs at <https://elastio.github.io/bon/reference/builder#start-fn>
        pub const name: Identifier = Identifier;

        /// See the docs at <https://elastio.github.io/bon/reference/builder#start-fn>
        pub const vis: VisibilityString = VisibilityString;
    }

    /// See the docs at <https://elastio.github.io/bon/reference/builder#finish-fn>
    pub const finish_fn: Option<Identifier> = None;

    /// See the docs at <https://elastio.github.io/bon/reference/builder#builder-type>
    pub const builder_type: Option<Identifier> = None;

    /// See the docs at <https://elastio.github.io/bon/reference/builder#expose-positional-fn>
    pub const expose_positional_fn: Option<Identifier> = None;

    /// See the docs at <https://elastio.github.io/bon/reference/builder#expose-positional-fn>
    pub mod expose_positional_fn {
        use super::*;

        /// See the docs at <https://elastio.github.io/bon/reference/builder#expose-positional-fn>
        pub const name: Identifier = Identifier;

        /// See the docs at <https://elastio.github.io/bon/reference/builder#expose-positional-fn>
        pub const vis: Option<VisibilityString> = None;
    }

    /// See the docs at <https://elastio.github.io/bon/reference/builder#on>
    pub mod on {
        use super::*;

        /// See the docs at <https://elastio.github.io/bon/reference/builder#on>
        pub const into: Flag = Flag;
    }

    /// See the docs at <https://elastio.github.io/bon/reference/builder#derive>
    pub mod derive {
        /// See the docs at <https://elastio.github.io/bon/reference/builder#derive>
        pub use core::fmt::Debug;

        /// See the docs at <https://elastio.github.io/bon/reference/builder#derive>
        pub use core::clone::Clone;
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
