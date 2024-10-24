use bon::{bon, builder, Builder};

#[derive(Builder)]
#[builder(start_fn())]
struct EmptyStartFn {}

#[derive(Builder)]
#[builder(finish_fn())]
struct EmptyFinisFn {}

#[derive(Builder)]
#[builder(start_fn)]
struct BareStartFnAttrOnStruct {}

#[builder(start_fn)]
fn bare_start_fn_on_free_function() {}

#[builder(start_fn())]
fn empty_paren_start_fn_on_free_function() {}

#[builder(start_fn(vis = ""))]
fn missing_name_for_start_fn_on_free_function1() {}

#[builder(start_fn(docs {}))]
fn missing_name_for_start_fn_on_free_function2() {}

struct AssocCtx;

#[bon]
impl AssocCtx {
    #[builder(start_fn)]
    fn bare_start_fn_on_non_new_method() {}
}

#[bon]
impl AssocCtx {
    #[builder(start_fn())]
    fn new() {}
}

#[bon]
impl AssocCtx {
    #[builder(start_fn(vis = ""))]
    fn missing_name_for_start_fn_on_non_new_method1() {}
}

#[bon]
impl AssocCtx {
    #[builder(start_fn(docs {}))]
    fn missing_name_for_start_fn_on_non_new_method2() {}
}

fn main() {}
