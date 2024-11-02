# `setters`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Overrides name, visibility and docs for setters.

The config is tree-structured with overrides precedence explained in the next paragraph.

```attr
#[builder(
    setters(
        name = custom_name,
        vis = "pub(crate)",
        doc {
            /// Custom docs for all setters
        },

        // There is short and long syntax (select only one)
        some_fn = custom_name,
        some_fn(
            name = custom_name,
            vis = "pub(crate)",
            doc {
                /// Custom docs for the `some_fn` setter
            }
        ),

        // There is short and long syntax (select only one)
        option_fn = custom_name,
        option_fn(
            name = custom_name,
            vis = "pub(crate)",
            doc {
                /// Custom docs for the `option_fn` setter
            }
        )
    )
)]
```

The main use case for this attribute is making generated setters private to wrap them with custom setters. See ["Builder extensions"](../../../guide/builder-extensions#custom-setters) for details.

## Config precedence

The keys `some_fn` and `option_fn` are available only for optional members that have a [pair of setters](../../../guide/optional-members#setters-pair).

The root-level `name`, `vis`, `docs` are still available for both required and optional setters. They can be overwritten at `some_fn` and `option_fn` level individually.

### Example

```rust
#[derive(bon::Builder)]
struct Example {
    #[builder(setters(name = foo, some_fn = bar))]
    member: Option<u32>,
}

// Name of `some_fn` that accepts the non-None value was overridden
Example::builder().bar(2).build();

// Name of `option_fn` was derived from the root-level `setters(name)`
Example::builder().maybe_foo(Some(2)).build();
```

## `name`

The default name for setters is chosen according to the following rules:

| Member type  | Default
|--------------|------------
| Required     | `{member}`
| Optional     | `some_fn` = `{member}`<br/>`option_fn` = `maybe_{member}`

This attribute is different from [`#[builder(name)]`](./name), because it overrides only the names of setters. It doesn't influence the name of the member in the builder's [typestate API](../typestate-api). This attribute also has higher precedence than [`#[builder(name)]`](./name).

## `vis`

The visibility must be enclosed with quotes. Use `""` or [`"pub(self)"`](https://doc.rust-lang.org/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself) for private visibility.

The default visibility is the same as the visibility of the [`builder_type`](../top-level/builder-type#vis), which in turn, defaults to the visibility of the underlying `struct` or `fn`.

## `doc`

Simple documentation is generated by default. The syntax of this attribute expects a block with doc comments.