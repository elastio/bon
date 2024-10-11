use crate::prelude::*;

#[test]
fn test_name() {
    #[derive(Builder)]
    #[builder(derive(Clone))]
    #[allow(dead_code)]
    struct Sut {
        #[builder(setters(name = arg1_renamed))]
        arg1: bool,

        #[builder(setters(name = arg2_renamed))]
        arg2: Option<()>,
    }

    let builder = Sut::builder().arg1_renamed(true);

    let _ = builder.clone().arg2_renamed(());
    let _ = builder.maybe_arg2_renamed(Some(()));
}
