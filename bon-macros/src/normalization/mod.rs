mod impl_traits;
mod lifetimes;
mod self_ty;
mod cfg;

pub(crate) use impl_traits::NormalizeImplTraits;
pub(crate) use lifetimes::NormalizeLifetimes;
pub(crate) use self_ty::NormalizeSelfTy;
pub(crate) use cfg::*;
