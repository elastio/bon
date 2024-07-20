use bon::builder;

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

fn main() {}
