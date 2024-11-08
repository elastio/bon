use bon::{bon, builder};

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

    #[builder(default = vec![1, 2, 3])] _arg5: Vec<u32>,
) {
    eprintln!("Non-const");
}

/// Function that returns a greeting special-tailored for a given person
#[builder(builder_type = GreeterBuilderCustom)]
pub fn greet(
    /// Name of the person to greet.
    ///
    /// **Example:**
    /// ```
    /// bon_sandbox::greet().name("John");
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
