#[rustversion::since(1.77.0)]
#[test]
#[expect(deprecated)]
fn builder_on_struct() {
    use crate::prelude::*;
    use core::net::IpAddr;

    #[builder]
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Sut {
        a: u32,

        #[builder(into)]
        b: IpAddr,

        #[builder(default)]
        c: u32,

        d: Option<u32>,

        #[builder(name = renamed)]
        e: u32,

        #[builder(skip = e + 99)]
        f: u32,
    }

    let actual = Sut::builder()
        .a(42)
        .b([127, 0, 0, 1])
        .maybe_d(None)
        .renamed(1)
        .build();

    assert_debug_eq(
        actual,
        expect!["Sut { a: 42, b: 127.0.0.1, c: 0, d: None, e: 1, f: 100 }"],
    );
}
