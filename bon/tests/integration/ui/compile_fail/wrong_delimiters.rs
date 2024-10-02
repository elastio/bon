#[derive(bon::Builder)]
#[builder(
    builder_type{},
    state_mod{},
    start_fn{},
    finish_fn{},
)]
struct CurlyBraces {}

#[derive(bon::Builder)]
struct CurlyBracesInField {
    #[builder(setters{})]
    x: u32,
}

#[derive(bon::Builder)]
#[builder(
    builder_type[docs[]],
    state_mod[docs[]],
    start_fn[docs[]],
    finish_fn[docs[]],
)]
struct SquareBrackets {
    #[builder(setters[])]
    x: u32,
}

#[derive(bon::Builder)]
struct SquareBracketsInField {
    #[builder(setters[])]
    x: u32,
}

#[derive(bon::Builder)]
#[builder(
    builder_type(docs[]),
    state_mod(docs[]),
    start_fn(docs[]),
    finish_fn(docs[]),
)]
struct SquareBracketsDocs {
    #[builder(setters(docs[]))]
    x: u32,
}

#[derive(bon::Builder)]
#[builder(
    builder_type(docs()),
    state_mod(docs()),
    start_fn(docs()),
    finish_fn(docs())
)]
struct Parentheses {
    #[builder(setters(docs()))]
    x: u32,
}

fn main() {}
