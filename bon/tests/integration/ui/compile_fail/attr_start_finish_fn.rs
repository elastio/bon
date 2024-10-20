use bon::Builder;

#[derive(Builder)]
#[builder(start_fn())]
struct EmptyStartFn {}

#[derive(Builder)]
#[builder(finish_fn())]
struct EmptyFinisFn {}

fn main() {}
