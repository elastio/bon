use bon::{bon, builder};

#[cfg(doctest)]
mod website_doctests {
    include!(concat!(env!("OUT_DIR"), "/website_doctests.rs"));
}

#[builder]
pub struct Greeter {
    _name: String,
    _level: usize,
}

pub struct Counter {
    val: usize,
}

#[bon]
impl Counter {
    /// Creates an instance of [`Self`] with an optional provided `initial` value.
    #[builder]
    pub fn new(
        /// Initial value for the counter.
        /// If not specified, defaults to 0.
        #[builder(default)]
        initial: usize,
    ) -> Self {
        Self { val: initial }
    }

    /// Increments the counter by `diff` amount. If not specified, increments by 1.
    #[builder]
    pub fn increment(
        &mut self,
        /// Amount to increment the counter by in [`Counter`].
        diff: Option<usize>,
    ) {
        self.val += diff.unwrap_or(1);
    }
}

/// Function-level documentation.
#[builder]
pub fn documented(
    /// Some documentation for the first argument
    ///
    /// # Doc test here
    ///
    /// ```
    /// // Some doc tests as well
    /// assert_eq!(2 + 2, 4);
    /// ```
    _arg1: String,

    _arg2: &str,

    _arg3: u32,

    _arg4: Vec<String>,
) {
}

/// Function that returns a greeting special-tailored for a given person
#[builder]
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
    format!("Hello {name} with age {age}!")
}

/// ```
/// #[derive(buildstructor::Builder)]
/// struct User {
///     name: String,
/// }
///
/// User::builder().name("Foo").build();
/// ```
///
fn foo() {}
