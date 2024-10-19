use bon::builder;

#[builder(on(String, into))]
fn unnecessary_into(#[builder(into)] _x: String) {}

#[builder(on(String, overwritable))]
fn unnecessary_ovewritable(#[builder(overwritable)] _x: String) {}

#[builder(on(&dyn std::fmt::Debug, into))]
fn invalid_type_pattern() {}

#[builder(on(fn(#[attr] a: u32), into))]
fn attrs_in_on_type_pattern() {}

#[builder(on)]
fn incomplete_on() {}

#[builder(on())]
fn incomplete_on2() {}

#[builder(on(_))]
fn incomplete_on3() {}

#[builder(on(_,))]
fn incomplete_on4() {}

fn main() {}
