struct Point {
    x: f64,
    y: f64,
}

#[builder(
    builder_type(
        name = ExampleBuilder,
        docs {
            /// Docs for the builder
        },
    ),

    state_mod(
        vis = "pub",
        name = name_override,
        docs {
            /// Docs for the state module
        },
        // Deprecate the state module
        deprecated = "..."
    ),

    on(
        Point,

        // (v3.3)
        //
        // Overrides the setters generated for the `Point` type.
        with = |x: f64, y: f64| Point { x, y },
    ),

    // (v3.4)
    //
    // Makes the builder exhaustive (all members must be set). Doesn't change
    // the setters that are generated, only adds more bounds on the `finish_fn`
    // to require all members to be set.
    //
    // Maybe there should be an alias for this pattern `#[builder(exhaustive)]`?
    // Will there be frequent use cases for this?
    on(_, required),

    // More predicates in `on(...)`
    on(not(required()), overwritable)
)]
pub struct Example {
    /// Docs on member
    #[builder(
        name = foo,

        // The `visibility` at this level may be confusing, because it doesn't
        // influence the the visibility of "state" items.
        //
        // (v3.0)
        // vis = "",

        // (v3.0)
        //
        // Override the name in the state
        state = PointOverride,

        // (v3.1+)
        //
        // Advanced state configuration
        // state(
        //     name = PointOverride,

        //     deprecated = "This state is no longer available, the field is overwritable",
        //     assoc_type = NameOverride,
        //     assoc_type(
        //         name = NameOverride
        //         docs {
        //             /// Docs for associated type
        //         },
        //     ),
        //     transition_alias = NameOverride,
        //     transition_alias(
        //         name = NameOverride,
        //         docs {
        //             /// Docs for transition alias
        //         },
        //     )
        // ),

        // Closure override
        //
        // For optional members, the `maybe_` setter will accept an `Option<(..args)>`
        // if there is more than one argument.
        with = |x: f64, y: f64| Point { x, y },

        // Makes the setter fallible
        with = |x: f64, y: f64| -> Result<_> { Ok(Point { x, y }) }

        // (v3.0)
        //
        // Overrides the name, visibility and docs of the default setters
        setters(
            name = point_internal,
            vis = "",

            // By default copied to all members
            docs {
                /// Docs for the setters
            }

            // (v3.1)
            //
            // Overrides specific to the `{member}` setter that wraps the value with `Some`
            //
            // Other names: `by_value`, `arg_value`, `plain`
            some_fn(
                name = point_internal,
                vis = "",
                docs { ... }
            ),

            // (v3.1)
            //
            // Overrides specific to the `maybe_{member}` setter.
            //
            // Other names: `by_option`, `arg_option`
            option_fn(
                name = maybe_point_internal,
                vis = "",
                docs { ... }
            )

            // Other names: `arg_absent`
            true_fn(
                // ...
            ),

            bool_fn(
                // ...
            )
        ),

        // (v3.2)
        //
        // These must work for regular members and start_fn args (custom fields?, v3.3)
        // Consider exposing `start_fn` args and overwritable optional fields as regular
        // private (!) fields on the builder additionally. This will allow for more flexibility
        // in the builder impl block.
        //
        // &T, Option<&T>
        getter,

        // (v3.2)
        //
        // &Option<T>
        getter(transparent),

        // (v3.2)
        // &mut T, Option<&mut T>
        getter(mut),

        // (v3.2)
        //
        // &mut Option<T>
        getter(transparent, mut)

        // (v3.2)
        //
        // Deref to &str, Option<&str>
        getter(deref(&str)),

        // (v3.2)
        //
        // Deref to &mut str, Option<&mut str>
        getter(deref(&mut str)),

        // (v3.2)
        //
        // AsRef to &str
        getter(as_ref(&str)),

        // (v3.2)
        //
        // `Option::as_ref() -> Option<&T>`
        getter(as_ref),

        // (v3.2)
        //
        // `<T as AsRef<_>>::as_ref() -> Option<&_>`
        getter(as_ref(&str)),


        // (v3.2)
        //
        // Getter by `Copy` -> `T`
        getter(copy),

        // (v3.2)
        //
        // Getter by `Clone` -> `T`
        getter(clone),

        getter(
            name = getter_name,
            vis = "",
            docs {
                /// Docs for getter_name
            }
            deprecated,
            copy,
        )

        // Multiple getters need to have name specified explicitly
        // getter(name = getter_name_1, copy),
        getter(name = getter_name_2),

        // Custom readonly getter. Accepts a readonly reference and transforms it.
        getter = |value: &_| -> Ty { expr }

        // Custom mutable getter. Accepts a mutable reference and transforms it.
        getter = |value: &mut _| -> Ty { expr }

        // Give a name to the getter if there are several getters
        getter(name = foo, with = |value: &mut _| -> Ty { expr }),
    )]
    point: Point,

    // v3.0
    #[builder(overwritable)]
    overwritable: u32,

    #[builder(
        field = vec![],
        field(name = overridden_name, vis = "pub", docs { ... }, init = vec![]),
        deprecated(reason = "saasd"),
    )]
    #[deprecated = "Use `overridden_name` instead"]
    pub custom_state: Vec<u32>,

    // Generates two setters for booleans:
    // - `my_lovely_flag() -> true`
    // - `with_my_lovely_flag(bool)`
    //
    // It also automatically implies that the setters are optional to call.
    // The default value is `false` automatically. The `#[builder(default)]`
    // attribute is not allowed with the `flag` attribute.
    #[builder(flag)]
    my_lovely_flag: bool,

    // The same as `#[builder(flag)]` but additionally requires the caller
    // to call the setter for this member explicitly.
    #[builder(flag, required)]
    my_required_flag: bool,

    // Opts out from the special handling for `Option<T>`. Generates only
    // a single setter that accepts `Option<T>` as a value. It's required
    // to call the setter.
    #[builder(transparent)]
    required_option: Option<String>,

    // Still generates a pair of setters (arg_value, arg_option), but requires
    // calling ant of these setters.
    #[builder(required)]
    exhaustive_option: Option<u32>,

    // Still generates a pair of setters (arg_value, arg_option), but requires
    // calling ant of these setters.
    #[builder(required, default = 32)]
    exhaustive_default: u32,
}

// Use cases:
struct UseCases {
    // (v3.0)
    //
    // Generate private setters with names `[maybe_]point_internal` and
    // preserve the public name in the `state` as `Point`.
    #[builder(setters(name = point_internal, vis = ""))]
    point: Point,

    // (v3.0)
    #[builder(setters(docs {
        /// Docs for the setters
    }))]
    override_docs_for_default_setters: Option<Point>,

    #[builder(setters(
        some_fn(docs {
            /// Docs for the some setter
        }),
        maybe_fn(docs {
            /// Docs for the maybe setter
        })
    ))]
    override_docs_for_maybe_setter: Option<Point>,

    // (v3.1)
    #[builder(with = |iter: impl IntoIterator<Item = String>| iter.into_iter().collect())]
    #[builder(with = |iter: impl IntoIterator<Item = String>| Vec::from_iter(iter))]
    take_into_iter: Vec<String>,

    // (v3.1)
    #[builder(
        setters(
            name = member_internal,
            vis = "",
            docs {
                /// ...
            }
        )
    )]
    take_several_args: Point,
}

impl<State: example_builder::State> ExampleBuilder<State> {
    pub fn my_point(self, x: f64, y: f64) -> ExampleBuilder<example_builder::SetPoint<State>>
    where
        State::Point: example_builder::IsUnset,
    {
        self.point(Point { x, y })
    }
}

#[bon]
impl Example {
    // Prevent shadowing the `new` function with the builder syntax.
    #[builder(separate)]
    fn new() {}
}

// A rename prevents shadowing the `example` function with the builder syntax
#[builder(start_fn = example_builder)]
fn example() {}
