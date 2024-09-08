// Please note that most parts of field variability in naming tested by getters
#[test]
fn compiles_and_works() {
    use crate::prelude::*;
    #[derive(bon::Builder)]
    #[allow(dead_code)]
    struct Sut<'e> {
        #[builder(getter, name = aaa)]
        a: i32,
        #[builder(getter)]
        b: String,
        c: Option<()>,
        #[builder(getter)]
        d: Option<[u8; 3]>,
        #[builder(getter)]
        e: &'e u8,
    }

    let abcde = Sut::builder().aaa(1);
    assert_eq!(abcde.get_aaa(), &1);

    let abcde = abcde.maybe_d(Some([1, 2, 3]));
    assert_eq!(abcde.get_d(), Some(&[1, 2, 3]));

    let abcde = abcde.b("hello".to_string());
    assert_eq!(abcde.get_b(), &"hello".to_string());

    let for_ref = 42;
    let abcde = abcde.e(&for_ref);
    let _e: &u8 = abcde.get_e();
}
