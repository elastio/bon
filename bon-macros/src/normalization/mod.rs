mod cfg;
mod impl_traits;
mod lifetimes;
mod self_ty;

pub(crate) use cfg::*;
pub(crate) use impl_traits::NormalizeImplTraits;
pub(crate) use lifetimes::NormalizeLifetimes;
pub(crate) use self_ty::NormalizeSelfTy;
