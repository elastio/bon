use bon::Builder;

#[derive(Builder)]
pub(crate) struct SomeFnSetterRequiredMember {
    #[builder(setters(some_fn = foo))]
    pub(crate) member: Option<i32>,
}

#[derive(Builder)]
pub(crate) struct OptionFnSetterOnRequiredMember {
    #[builder(setters(option_fn = bar))]
    pub(crate) member: Option<i32>,
}

fn main() {}
