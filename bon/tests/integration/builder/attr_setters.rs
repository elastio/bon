use crate::prelude::*;

#[test]
fn test_name() {
    #[derive(Builder)]
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

    let _ = Sut::builder().arg1_renamed(true);

    let _ = Sut::builder().arg2_renamed(());
    let _ = Sut::builder().maybe_arg2_renamed(Some(()));

    let _ = Sut::builder().arg3_renamed(42);
    let _ = Sut::builder().maybe_arg3_renamed(Some(42));

    // The name in the state must remain the same
    let _: SutBuilder<SetArg3<SetArg2<SetArg1>>> = Sut::builder()
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

#[test]
fn test_option_fn_name_and_some_fn_name() {
    #[derive(Builder)]
    #[builder(derive(Clone))]
    #[allow(dead_code)]
    struct Sut {
        #[builder(setters(some_fn = arg1_some))]
        arg1: Option<()>,

        #[builder(setters(option_fn = arg2_option))]
        arg2: Option<()>,

        #[builder(setters(some_fn = arg3_some, option_fn = arg3_option))]
        arg3: Option<()>,

        #[builder(setters(some_fn(name = arg4_some), option_fn(name = arg4_option)))]
        arg4: Option<()>,

        #[builder(default, setters(some_fn = arg5_some))]
        arg5: (),

        #[builder(default, setters(option_fn = arg6_option))]
        arg6: (),

        #[builder(default, setters(some_fn = arg7_some, option_fn = arg7_option))]
        arg7: (),

        #[builder(default, setters(some_fn(name = arg8_some), option_fn(name = arg8_option)))]
        arg8: (),
    }

    use sut_builder::*;

    let _ = Sut::builder().arg1_some(());
    let _ = Sut::builder().maybe_arg1(Some(()));

    let _ = Sut::builder().arg2(());
    let _ = Sut::builder().arg2_option(Some(()));

    let _ = Sut::builder().arg3_some(());
    let _ = Sut::builder().arg3_option(Some(()));

    let _ = Sut::builder().arg4_some(());
    let _ = Sut::builder().arg4_option(Some(()));

    let _ = Sut::builder().arg5_some(());
    let _ = Sut::builder().maybe_arg5(Some(()));

    let _ = Sut::builder().arg6(());
    let _ = Sut::builder().arg6_option(Some(()));

    let _ = Sut::builder().arg7_some(());
    let _ = Sut::builder().arg7_option(Some(()));

    let _ = Sut::builder().arg8_some(());
    let _ = Sut::builder().arg8_option(Some(()));

    #[allow(clippy::type_complexity)]
    let _: SutBuilder<SetArg8<SetArg7<SetArg6<SetArg5<SetArg4<SetArg3<SetArg2<SetArg1>>>>>>>> =
        Sut::builder()
            .arg1_some(())
            .arg2(())
            .arg3_some(())
            .arg4_some(())
            .arg5_some(())
            .arg6(())
            .arg7_some(())
            .arg8_some(());
}
