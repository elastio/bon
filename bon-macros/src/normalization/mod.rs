mod cfg;
mod impl_traits;
mod lifetimes;
mod self_ty;

pub(crate) use cfg::*;
pub(crate) use impl_traits::NormalizeImplTraits;
pub(crate) use lifetimes::NormalizeLifetimes;
pub(crate) use self_ty::NormalizeSelfTy;

/// Struct, that contains both the original syntax (unprocessed) and the normalized
/// version. This is useful for code that needs access to both versions of the syntax.
#[derive(Debug)]
pub(crate) struct SyntaxVariant<T> {
    /// Original syntax that was passed to the macro without any modifications.
    pub(crate) orig: T,

    /// The value that is equivalent to `orig`, but it underwent normalization.
    pub(crate) norm: T,
}

impl<T> SyntaxVariant<T> {
    pub(crate) fn apply_ref<'a, U>(&'a self, f: impl Fn(&'a T) -> U) -> SyntaxVariant<U> {
        let orig = f(&self.orig);
        let norm = f(&self.norm);
        SyntaxVariant { orig, norm }
    }
}
