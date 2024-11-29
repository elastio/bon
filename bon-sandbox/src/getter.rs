use bon::{builder, Builder};

#[derive(Builder)]
pub struct FullName {
    #[builder(getter)]
    pub first_name: String,
    #[builder(getter(name = get_the_last_name, vis = "pub(crate)", doc {
        /// Docs on the getter
    }))]
    pub last_name: String,
    pub no_getter: String,
}
