#![cfg_attr(not(feature = "std"), no_std)]
#![allow(
    clippy::redundant_pub_crate,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_value,
    clippy::too_many_lines,
    non_local_definitions
)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod prelude {
    #[cfg(feature = "alloc")]
    pub(crate) use alloc::{
        borrow::ToOwned, collections::BTreeSet, format, string::String, vec, vec::Vec,
    };

    pub(crate) use super::assert_debug_eq;
    pub(crate) use bon::{bon, builder};
    pub(crate) use expect_test::expect;
}

mod builder;

mod ui;

use expect_test::Expect;

/// Approximate number of characters that can fit on a single screen
const COMMON_SCREEN_CHARS_WIDTH: usize = 60;

#[track_caller]
#[allow(clippy::needless_pass_by_value)]
fn assert_debug_eq(actual: impl core::fmt::Debug, expected: Expect) {
    extern crate alloc;

    let snapshot = 'snap: {
        let terse = alloc::format!("{actual:?}");

        let Some(width) = terse.lines().map(str::len).max() else {
            break 'snap terse;
        };

        if width < COMMON_SCREEN_CHARS_WIDTH {
            break 'snap terse;
        }

        alloc::format!("{actual:#?}")
    };

    expected.assert_eq(&snapshot);
}
