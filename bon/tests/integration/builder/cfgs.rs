#![allow(clippy::non_minimal_cfg)]

use crate::prelude::*;

#[test]
fn struct_smoke() {
    #[derive(Debug, Builder)]
    struct Sut {
        #[cfg(all())]
        #[cfg_attr(all(), allow(dead_code))]
        arg1: bool,

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
    #[builder(builder_type = OverrideBuilder)]
    #[allow(dead_code)]
    struct Sut {
        #[cfg(all())]
        arg1: bool,

        #[cfg(not(all()))]
        arg1: u32,

        #[cfg_attr(all(), builder(default))]
        arg2: [u8; 4],

        #[cfg_attr(any(), builder(name = renamed))]
        arg3: [char; 2],
    }

    assert_debug_eq(
        Sut::builder()
            .arg1(true)
            // arg3 is not renamed
            .arg3(['a', 'b'])
            .build(),
        expect!["Sut { arg1: true, arg2: [0, 0, 0, 0], arg3: ['a', 'b'] }"],
    );
}

#[test]
fn fn_smoke() {
    #[builder]
    fn sut(
        #[cfg(not(all()))]
        #[cfg_attr(all(), allow(dead_code))]
        arg1: bool,

        #[cfg(all())] arg1: u32,

        #[cfg(any())] arg1: String,
    ) -> u32 {
        arg1.saturating_mul(2)
    }

    assert_debug_eq(sut().arg1(32).call(), expect!["64"]);
}

#[test]
fn fn_with_params() {
    #[derive(Builder)]
    #[builder(builder_type = OverrideBuilder)]
    #[allow(dead_code)]
    struct Sut {
        #[cfg(all())]
        arg1: bool,

        #[cfg(not(all()))]
        arg1: u32,

        #[cfg_attr(all(), builder(default))]
        arg2: [u8; 4],

        #[cfg_attr(any(), builder(name = renamed))]
        arg3: [char; 2],
    }

    let _ = Sut::builder()
        .arg1(true)
        // arg3 is not renamed
        .arg3(['a', 'b'])
        .build();
}
