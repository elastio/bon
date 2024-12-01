use bon::{builder, Builder};

#[builder]
pub fn full_name_fn(#[builder(getter)] first_name: &str, last_name: &str) -> String {
    format!("{first_name} {last_name}")
}

#[derive(Builder)]
pub struct FullName {
    #[builder(getter)]
    first_name: String,

    #[builder(getter(name = get_the_last_name, vis = "pub(crate)", doc {
        /// Docs on the getter
    }))]
    last_name: String,

    no_getter: String,
}
