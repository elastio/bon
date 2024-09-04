#![allow(clippy::non_minimal_cfg)]

use crate::prelude::*;
use bon::builder;

#[test]
fn struct_smoke() {
    #[builder]
    #[derive(Debug)]
    struct Sut {
        #[cfg(all())]
        #[cfg_attr(all(), allow(dead_code))]
        arg1: bool,

        #[cfg(not(all()))]
        arg1: u32,
    }

    assert_debug_eq(Sut::builder().arg1(true).build(), expect![]);
}

#[test]
fn struct_with_params() {
    #[builder(builder_type = OverrideBuilder)]
    struct Sut {
        #[cfg(all())]
        arg1: bool,

        #[cfg(not(all()))]
        arg1: u32,
    }

    let _ = Sut::builder()
        .arg1(true)
        .build();
}
