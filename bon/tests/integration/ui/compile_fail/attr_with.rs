use bon::Builder;

#[derive(Builder)]
struct ConflictingInto {
    #[builder(into, with = |x: u32| x + 1)]
    value: u32,
}

fn main() {}
