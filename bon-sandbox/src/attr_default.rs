#[derive(bon::Builder)]
#[builder(builder_type(
    doc {
        /// Showcases examples of `#[builder(default)]` usage and what docs are
        /// generated for them. Click on any of `source` links to see the source code.
    }
))]
pub struct Example {
    #[builder(default = (2 + 2) * 10)]
    small_custom_default: u32,

    #[builder(default = Vec::from([
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: 5, y: 6 },
        Point { x: 7, y: 8 },
    ]))]
    big_custom_default: Vec<Point>,

    #[builder(default)]
    standard_u32_default: u32,

    #[builder(default)]
    standard_string_default: String,
}

struct Point {
    x: u32,
    y: u32,
}
