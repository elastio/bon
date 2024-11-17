use crate::prelude::*;

#[test]
fn test_struct() {
    #[derive(Builder)]
    #[builder(derive(Debug, Clone))]
    struct Sut {
        #[builder(start_fn)]
        x1: u32,

        #[builder(start_fn, into)]
        x2: u32,

        #[builder(field)]
        x3: bool,

        #[builder(field = x1 + x2 + u32::from(x3))]
        x4: u32,

        #[builder(field = x4 + 1)]
        x5: u32,

        x6: (),
    }

    let sut = Sut::builder(1, 2_u32);

    assert_eq!(sut.x3, false);
    assert_eq!(sut.x4, 3);
    assert_eq!(sut.x5, 4);
}
