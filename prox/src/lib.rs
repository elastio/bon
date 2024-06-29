//! This crate accumulates various utilities that are shared between proc macro
//! crates. If you see any pattern repated in proc macros, feel free to move
//! that to this shared place.
extern crate proc_macro;

mod attrs;
mod entrypoint;
mod expr_ext;
mod ident_ext;
mod fn_arg_ext;
mod path_ext;
mod printing;
mod type_ext;
mod visibility_ext;

#[cfg(feature = "test")]
pub mod test;

pub use attrs::*;
pub use entrypoint::*;
pub use ident_ext::*;
pub use printing::*;

pub mod prelude {
    pub use crate::attrs::{AttributeExt, MetaListExt};
    pub use crate::expr_ext::ExprExt;
    pub use crate::ident_ext::IdentExt;
    pub use crate::path_ext::PathExt;
    pub use crate::type_ext::TypeExt;
    pub use crate::fn_arg_ext::FnArgExt;
    pub use crate::visibility_ext::VisibilityExt;
}

#[doc(hidden)]
pub mod imp {
    pub use darling;
}

pub type Result<T, E = darling::Error> = std::result::Result<T, E>;

/// Inspired by `anyhow::bail`, but returns a [`Result`] with [`darling::Error`].
/// It accepts the value that implements [`syn::spanned::Spanned`] to attach the
/// span to the error.
#[macro_export]
macro_rules! bail {
    ($spanned:expr, $($tt:tt)*) => {
        return Err($crate::err!($spanned, $($tt)*))
    };
}

/// Inspired by `anyhow::anyhow`, but returns a [`darling::Error`].
/// It accepts the value that implements [`syn::spanned::Spanned`] to attach the
/// span to the error.
#[macro_export]
macro_rules! err {
    ($spanned:expr, $($tt:tt)*) => {
        $crate::imp::darling::Error::custom(format_args!($($tt)*)).with_span($spanned)
    };
}
