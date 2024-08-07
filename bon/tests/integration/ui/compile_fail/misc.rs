use bon::builder;
use std::collections::{BTreeMap, BTreeSet};

#[builder]
struct TupleStruct(u32, u32);

#[builder]
fn destructuring((x, y): (u32, u32)) {
    let _ = x;
    let _ = y;
}

#[builder]
fn explicit_into_equals_true(#[builder(into = true)] _x: u32) {}

#[builder]
fn unnecessary_into_override_true(#[builder(into)] _x: String) {}

#[builder]
fn unnecessary_into_override_false(#[builder(into = false)] _x: u32) {}

#[builder(start_fn())]
struct EmptyStartFn {}

fn main() {
    let map: BTreeMap<String, String> = bon::map! {
        "Hello": "Blackjack",
        "Hello": "Littlepip",
    };

    let set: BTreeSet<String> = bon::set!["mintals", "guns", "mintals", "roses"];
}
