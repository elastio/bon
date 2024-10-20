//! This crate is used only for testing of the public interface of the `bon` crate.
//! We don't need all the aggressive lints that we use for public crates.
#![allow(missing_debug_implementations, missing_docs)]

pub mod attr_with;
pub mod macro_rules_wrapper_test;
pub mod missing_docs_test;
pub mod state_mod_pub;

mod reexports;

pub use reexports::{UnexportedBuilder, UnexportedStateMod, UnexportedStateModBuilder};

use bon::{bon, builder, Builder};

#[cfg(doctest)]
// We use a bunch of Vitepress-specific syntax in the doctests, for example to
// give a name to a code group in a fenced code block, which conflicts with this
// lint.
#[deny(rustdoc::invalid_codeblock_attributes)]
mod website_doctests {
    include!(concat!(env!("OUT_DIR"), "/website_doctests.rs"));
}

/// Some docs on the private builder
#[derive(Builder)]
#[builder(builder_type(vis = ""))]
pub struct PrivateBuilder {
    _field: String,
}

/// Docs on the [`Self`] struct
#[derive(Builder)]
#[builder(
    builder_type(
        doc {
            /// Docs on [`GreeterOverriddenBuilder`]
            /// the builder type
        },
        name = GreeterOverriddenBuilder,
    ),
    start_fn(
        doc {
            /// Docs on
            /// [`Self::start_fn_override`]
        },
        name = start_fn_override,
    ),
    finish_fn(
        doc {
            /// Docs on
            /// [`GreeterOverriddenBuilder::finish_fn_override()`]
        },
        name = finish_fn_override,
    )
)]
pub struct Greeter {
    /// Docs on
    /// the `name` field
    _name: String,

    /// Docs on
    /// the `level` field
    _level: usize,
}

pub struct Counter {
    val: usize,
}

#[bon]
impl Counter {
    #[builder]
    pub fn new(
        /// Initial value for the counter.
        /// If not specified, defaults to 0.
        #[builder(default)]
        initial: usize,
    ) -> Self {
        eprintln!("Non-const");
        Self { val: initial }
    }

    /// Increments the counter by `diff` amount. If not specified, increments by 1.
    #[builder]
    pub fn increment(
        &mut self,
        /// Amount to increment the counter by in [`Counter`].
        diff: Option<usize>,
    ) {
        eprintln!("Non-const");
        self.val += diff.unwrap_or(1);
    }
}

/// Function-level documentation.
#[builder]
#[allow(clippy::needless_pass_by_value)]
pub fn documented(
    /// Some documentation for the first argument
    ///
    /// # Doc test here
    ///
    /// ```
    /// // Some doc tests as well
    /// assert_eq!(2 + 2, 4);
    /// ```
    #[builder(default)]
    _arg1: String,

    _arg2: &str,

    /// Optional member docs
    _arg3: Option<u32>,

    _arg4: Vec<String>,

    #[builder(default =
        Greeter::start_fn_override()
            .name(
                "Some intentionally big expression to test the fallback to \
                a code fence in the default value docs"
                .to_owned()
            )
            .level(42)
            .finish_fn_override()
    )]
    _arg5: Greeter,
) {
    eprintln!("Non-const");
}

/// Function that returns a greeting special-tailored for a given person
#[builder(builder_type = Foo)]
pub fn greet(
    /// Name of the person to greet.
    ///
    /// **Example:**
    /// ```
    /// e2e_tests::greet().name("John");
    /// ```
    name: &str,

    /// Age expressed in full years passed since the birth date.
    age: u32,
) -> String {
    eprintln!("Non-const");
    format!("Hello {name} with age {age}!")
}

#[builder]
pub fn fn_with_impl_trait(_arg1: impl std::fmt::Debug + Clone, _arg2: impl std::fmt::Debug) {}

#[builder]
pub fn many_function_parameters(
    _id: Option<&str>,
    _keyword: Option<&str>,
    _attraction_id: Option<&str>,
    _venue_id: Option<&str>,
    _postal_code: Option<&str>,
    _latlong: Option<&str>,
    _radius: Option<&str>,
    _unit: Option<&str>,
    _source: Option<&str>,
    _locale: Option<&str>,
    _market_id: Option<&str>,
    _start_date_time: Option<&str>,
    _end_date_time: Option<&str>,
    _include_tba: Option<&str>,
    _include_tbd: Option<&str>,
    _include_test: Option<&str>,
    _size: Option<&str>,
    _page: Option<&str>,
    _sort: Option<&str>,
    _onsale_start_date_time: Option<&str>,
    _onsale_end_date_time: Option<&str>,
    _city: Option<&str>,
    _country_code: Option<&str>,
    _state_code: Option<&str>,
    _classification_name: Option<&str>,
    _classification_id: Option<&str>,
    _dma_id: Option<&str>,
    _onsale_on_start_date: Option<&str>,
    _onsale_on_after_start_date: Option<&str>,
    _segment_id: Option<&str>,
    _segment_name: Option<&str>,
    _promoter_id: Option<&str>,
    _client_visibility: Option<&str>,
    _nlp: Option<&str>,
    _include_licensed_content: Option<&str>,
    _geopoint: Option<&str>,
) {
    eprintln!("Non-const");
}
