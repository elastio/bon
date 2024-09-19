pub trait Members {
    type Members;
}

bon_macros::__gen_tuple_traits2!(100);

type N = never::Never;

mod never {
    /// Unnameable type. It's public, but under a private module.
    #[doc(hidden)]
    #[allow(unreachable_pub, unnameable_types, missing_debug_implementations)]
    pub enum Never {}
}
