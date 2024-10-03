use bon::Builder;

#[derive(Builder)]
struct RegularMember {
    #[builder(transparent)]
    member: i32,
}

#[derive(Builder)]
struct StartFnMember {
    #[builder(transparent)]
    member: Option<i32>,
}

#[derive(Builder)]
struct FinishFnMember {
    #[builder(transparent)]
    member: Option<i32>,
}

fn main() {}
