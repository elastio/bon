use buildy::builder;

fn foo(a: u32, b: bool) {
    let output = compute_stuff1()
        .arg1(32)
        .arg2("asd".to_owned())
        // .arg3(Some(true))
        .arg4(vec!["a".to_owned(), "b".to_owned()])
        .call();
}

#[builder]
fn empty() {}

#[builder]
fn compute_stuff1(arg1: u32, arg2: String, arg3: Option<bool>, arg4: Vec<String>) -> String {
    // compute_stuff().foo(&23);

    // let foo = arg1;
    "Hello, world!".to_string()
}

/// Some documentation here
#[builder]
fn compute_stuff<'a, T, U, B: IntoIterator>(
    /// Arg documentation
    /// Multi-line
    foo: &'a u32,

    /// Arg documentation 2
    /// Multi-line 2
    u: U,

    b: B::Item
) -> String
where
    T: Default,
    'a: 'static,
{
    // compute_stuff().foo(&23);

    // let foo = arg1;
    "Hello, world!".to_string()
}
