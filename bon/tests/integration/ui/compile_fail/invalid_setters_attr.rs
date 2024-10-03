use bon::Builder;

#[derive(Builder)]
struct SomeFnSetterRequiredMember {
    #[builder(setters(some_fn = foo))]
    member: i32,
}

#[derive(Builder)]
struct OptionFnSetterOnRequiredMember {
    #[builder(setters(option_fn = bar))]
    member: i32,
}

#[derive(Builder)]
struct SomeFnSetterWithTransparent {
    #[builder(transparent, setters(some_fn = foo))]
    member: Option<i32>,
}

#[derive(Builder)]
struct OptionFnSetterWithTransparent {
    #[builder(transparent, setters(option_fn = bar))]
    member: Option<i32>,
}

fn main() {}
