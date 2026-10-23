#![allow(clippy::non_minimal_cfg)]

use crate::prelude::*;

#[test]
fn struct_smoke() {
    #[derive(Debug, Builder)]
    struct Sut {
        #[cfg(all())]
        #[cfg_attr(all(), allow(dead_code))]
        arg1: bool,

        /// doc comment
        #[cfg(not(all()))]
        arg1: u32,

        #[cfg(any())]
        arg1: String,
    }

    assert_debug_eq(
        Sut::builder().arg1(true).build(),
        expect!["Sut { arg1: true }"],
    );
}

#[test]
fn struct_with_params() {
    #[derive(Debug, Builder)]
    #[cfg_attr(all(), builder(builder_type = OverrideBuilder, finish_fn = finish))]
    #[cfg_attr(any(), builder(builder_type = Unreachable))]
    #[allow(dead_code)]
    struct Sut {
        #[cfg(all())]
        arg1: bool,

        /// doc comment
        #[cfg(not(all()))]
        arg1: u32,

        #[cfg_attr(all(), builder(default))]
        arg2: [u8; 4],

        #[cfg_attr(any(), builder(name = renamed))]
        arg3: [char; 2],
    }

    let builder: OverrideBuilder = Sut::builder();

    assert_debug_eq(
        builder
            .arg1(true)
            // arg3 is not renamed
            .arg3(['a', 'b'])
            .finish(),
        expect!["Sut { arg1: true, arg2: [0, 0, 0, 0], arg3: ['a', 'b'] }"],
    );
}

#[test]
fn fn_smoke() {
    #[builder]
    fn sut(
        #[cfg(all())]
        #[cfg_attr(all(), allow(dead_code))]
        arg1: bool,

        /// doc comment
        #[cfg(not(all()))]
        arg1: u32,

        #[cfg(any())] arg1: String,
    ) -> bool {
        arg1
    }

    assert!(sut().arg1(true).call());
}

#[test]
fn fn_with_params() {
    #[cfg_attr(all(), builder(builder_type = OverrideBuilder))]
    #[cfg_attr(any(), builder(builder_type = Unreachable))]
    fn sut(
        #[cfg(all())] arg1: bool,

        /// doc comment
        #[cfg(not(all()))]
        arg1: u32,

        #[cfg_attr(all(), builder(default))] arg2: [u8; 4],

        #[cfg_attr(any(), builder(name = renamed))] arg3: [char; 2],
    ) -> (bool, [u8; 4], [char; 2]) {
        (arg1, arg2, arg3)
    }

    let builder: OverrideBuilder = sut();

    assert_debug_eq(
        builder.arg1(true).arg3(['a', 'b']).call(),
        expect!["(true, [0, 0, 0, 0], ['a', 'b'])"],
    );
}

#[test]
fn fn_with_nested_cfgs() {
    #[builder]
    fn sut(#[cfg_attr(all(), cfg_attr(not(not(all())), builder(name = renamed)))] arg: u32) -> u32 {
        arg
    }

    assert_eq!(sut().renamed(99).call(), 99);
}

#[test]
fn impl_block() {
    struct Sut;

    #[bon::bon]
    impl Sut {
        #[builder]
        fn sut_smoke(
            #[cfg(all())]
            #[cfg_attr(all(), allow(dead_code))]
            arg1: bool,

            /// doc comment
            #[cfg(not(all()))]
            arg1: u32,

            #[cfg(any())] arg1: String,
        ) -> bool {
            arg1
        }

        #[cfg_attr(all(), builder(builder_type = OverrideBuilder))]
        #[cfg_attr(any(), builder(builder_type = Unreachable))]
        fn sut_with_params(
            #[cfg(all())] arg1: bool,

            /// doc comment
            #[cfg(not(all()))]
            arg1: u32,

            #[cfg_attr(all(), builder(default))] arg2: [u8; 4],

            #[cfg_attr(any(), builder(name = renamed))] arg3: [char; 2],
        ) -> bool {
            let _ = (arg2, arg3);
            arg1
        }
    }

    assert!(Sut::sut_smoke().arg1(true).call());

    let builder: OverrideBuilder = Sut::sut_with_params();

    assert!(builder.arg1(true).arg3(['a', 'b']).call());
}

// The cfg evaluation generates identifiers from the name of the function or
// from the names of the `impl` block's self type and its first method. The `r#`
// prefix of raw identifiers must not leak into them, otherwise the generated
// identifiers are invalid (e.g. `__eval_cfg_bon_builder_r#type_0_0`), and the
// macro panics.
#[test]
fn raw_idents() {
    {
        #[builder]
        fn r#type(#[cfg(all())] r#type: u32, #[cfg(any())] r#while: u32) -> u32 {
            r#type
        }

        assert_eq!(r#type().r#type(42).call(), 42);
    }

    {
        struct r#Type;

        #[bon::bon]
        impl r#Type {
            #[builder]
            fn r#type(#[cfg(all())] r#type: u32, #[cfg(any())] r#while: u32) -> u32 {
                r#type
            }
        }

        assert_eq!(r#Type::r#type().r#type(42).call(), 42);
    }
}
