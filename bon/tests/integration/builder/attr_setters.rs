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

        #[builder(default, setters(name = arg3_renamed))]
        arg3: u32,
    }

    use sut_builder::*;

    let builder = Sut::builder().arg1_renamed(true);

    let _ = builder.clone().arg2_renamed(());
    let _ = builder.clone().maybe_arg2_renamed(Some(()));

    let _ = builder.clone().arg3_renamed(42);
    let _ = builder.maybe_arg3_renamed(Some(42));

    // The name in the state must remain the same
    let _: SutBuilder<SetArg1<SetArg3<SetArg2>>> = Sut::builder()
        .arg1_renamed(true)
        .arg2_renamed(())
        .arg3_renamed(42);

    #[allow(clippy::items_after_statements)]
    fn _assert_assoc_type_name<T: State>(_: T)
    where
        T::Arg1:,
        T::Arg2:,
    {
    }
}
