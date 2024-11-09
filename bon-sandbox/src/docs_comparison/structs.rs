/// Example docs generated with `bon`
pub mod bon {
    /// Doc comment on `Struct`
    #[derive(bon::Builder)]
    pub struct Struct {
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
    }
}

/// Example docs generated with `buildstructor`
pub mod buildstructor {
    /// Doc comment on `Struct`
    #[derive(buildstructor::Builder)]
    pub struct Struct {
        /// Doc comment on `x1`
        x1: u32,

        /// Doc comment on `x2`
        x2: u32,

        /// Doc comment on `x3`
        x3: Option<u32>,

        /// Doc comment on `x4`
        x4: String,
    }
}

/// Example docs generated with `typed-builder`
pub mod typed_builder {

    /// Doc comment on `Struct`
    #[derive(typed_builder::TypedBuilder)]
    #[builder(doc)]
    pub struct Struct {
        /// Doc comment on `x1`
        x1: u32,

        /// Doc comment on `x2`
        #[builder(default = 2 + 2)]
        x2: u32,

        /// Doc comment on `x3`
        #[builder(default)]
        x3: Option<u32>,

        /// Doc comment on `x4`
        #[builder(setter(into))]
        x4: String,
    }
}

/// Example docs generated with `derive_builder`
pub mod derive_builder {
    /// Doc comment on `Struct`
    #[derive(derive_builder::Builder)]
    pub struct Struct {
        /// Doc comment on `x1`
        x1: u32,

        /// Doc comment on `x2`
        #[builder(default = 2 + 2)]
        x2: u32,

        /// Doc comment on `x3`
        #[builder(default)]
        x3: Option<u32>,

        /// Doc comment on `x4`
        #[builder(setter(into))]
        x4: String,
    }
}
