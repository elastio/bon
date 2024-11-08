/// Docs on the [`Self`] struct
#[derive(bon::Builder)]
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
