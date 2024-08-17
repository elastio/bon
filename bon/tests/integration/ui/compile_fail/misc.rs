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

#[builder(setters())]
fn empty_setters() {}

#[builder(setters(into, filter(fn(#[attr] a: u32))))]
fn attrs_in_setters() {}

#[builder(start_fn())]
struct EmptyStartFn {}

#[builder]
struct ConflictingAttrs {
    #[builder(skip, into)]
    x: u32,
}

#[builder]
struct ConflictingAttrs2 {
    #[builder(skip, name = bar)]
    x: u32,
}

#[builder]
struct ConflictingAttrs3 {
    #[builder(skip, default = 42)]
    z: u32,
}

fn main() {
    let map: BTreeMap<String, String> = bon::map! {
        "Hello": "Blackjack",
        "Hello": "Littlepip",
    };

    let set: BTreeSet<String> = bon::set!["mintals", "guns", "mintals", "roses"];
}
