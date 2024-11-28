use bon::{builder, Builder};

#[derive(Builder)]
pub struct FullName {
    #[builder(getter(name = get_first_name))]
    pub first_name: String,
    pub last_name: String,
}

fn test_me() {
    let builder_with_first_name = FullName::builder().first_name(String::new());

    let first_name = builder_with_first_name.get_first_name();
}
