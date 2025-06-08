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
// This lint comes from buildstructor-generated code
#[expect(elided_lifetimes_in_paths)]
// This lint comes from nightly
#[allow(unknown_lints, mismatched_lifetime_syntaxes)]
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
