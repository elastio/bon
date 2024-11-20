/// Example docs generated with `bon`
pub mod bon {
    /// Doc comment on `Struct`
    pub struct Struct;

    #[bon::bon]
    impl Struct {
        /// Doc comment on `method`
        #[builder]
        pub fn method(
            &self,

            /// Doc comment on `x1`
            x1: u32,

            /// Doc comment on `x2`
            #[builder(default = 2 + 2)]
            x2: u32,

            /// Doc comment on `x3`
            x3: Option<u32>,

            /// Doc comment on `x4`
            #[builder(into)]
            x4: String,
        ) {
        }
    }
}

/// Example docs generated with `buildstructor`
#[expect(elided_lifetimes_in_paths)]
// `buildstructor` generates a `#[cfg_attr(feature = "cargo-clippy", ...)]`,
// which is not known to rustc. This lint comes from `nightly` at the time
// of this writing, so using `allow` for now instead of `expect`
#[allow(unexpected_cfgs)]
pub mod buildstructor {

    /// Doc comment on `Struct`
    pub struct Struct;

    #[buildstructor::buildstructor]
    impl Struct {
        /// Doc comment on `method_orig`
        #[builder(entry = "method")]
        pub fn method_orig(&self, x1: u32, x2: u32, x3: Option<u32>, x4: String) {}
    }
}
