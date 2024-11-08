#![allow(dead_code)]

pub mod bon {
    #[derive(bon::Builder)]
    pub struct Example {
        x1: u32,

        #[builder(default = 2 + 2)]
        x2: u32,

        x3: Option<u32>,

        #[builder(into)]
        x4: String,
    }
}

pub mod buildstructor {
    #[derive(buildstructor::Builder)]
    pub struct Example {
        x1: u32,
        x2: u32,
        x3: Option<u32>,
        x4: String,
    }
}

pub mod typed_builder {
    #[derive(typed_builder::TypedBuilder)]
    #[builder(doc)]
    pub struct Example {
        x1: u32,

        #[builder(default = 2 + 2)]
        x2: u32,

        #[builder(default)]
        x3: Option<u32>,

        #[builder(setter(into))]
        x4: String,
    }
}

pub mod derive_builder {
    #[derive(derive_builder::Builder)]
    pub struct Example {
        x1: u32,

        #[builder(default = 2 + 2)]
        x2: u32,

        #[builder(default)]
        x3: Option<u32>,

        #[builder(setter(into))]
        x4: String,
    }
}
