use buildy::builder;

fn foo() {
    // let output = compute_stuff()
    //     .arg1(32)
    //     .arg2("asd".to_owned())
    //     .arg3(Some(true))
    //     .arg4(vec!["a".to_owned(), "b".to_owned()])
    //     .call();
}

// /// Some documentation here
#[builder]
fn compute_stuff<'a, T>(foo: &'a u32) -> String {
    // compute_stuff().foo(&23);

    // let foo = arg1;
    "Hello, world!".to_string()
}
