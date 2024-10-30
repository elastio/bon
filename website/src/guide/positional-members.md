# Positional Members

While having the ability to use separate setters for the members gives you a ton of flexibility and extensibility described on the ["Compatibility"](./compatibility) page, sometimes you don't need all of that.

Maybe you'd like to pick out some specific members and let the user pass their values as positional parameters to the starting function that creates the builder or to the finishing function that consumes it. This reduces the syntax a bit at the cost of some extensibility loss ⚖️, but it may be worth it!

## Starting function

As an example, suppose we have a `Treasure` struct with `x` and `y` coordinates and a `label` that describes the payload of the treasure. Since all treasures are located somewhere, they all have coordinates, and it would be cool to specify them in a single starting function call.

To do that we can use the `#[builder(start_fn)]` attribute. There are two contexts where we can place it, and they both have a different meaning:

- [Top-level `#[builder(start_fn = ...)]`](../reference/builder/top-level/start-fn) - configures the name, visibility and docs of the starting function
- [Member-level `#[builder(start_fn)]`](../reference/builder/member/start-fn) - configures the member to be a positional parameter on the starting function

We'll want to use both of these attributes in our example to give a better name for the starting function that describes its inputs and configure `x` and `y` as positional parameters on the starting function as well.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
// Top-level attribute to give a better name for the starting function // [!code highlight]
#[builder(start_fn = with_coordinates)]                                // [!code highlight]
struct Treasure {
    // Member-level attributes to mark members as // [!code highlight]
    // parameter of `with_coordinates()`          // [!code highlight]
    #[builder(start_fn)]                          // [!code highlight]
    x: u32,

    #[builder(start_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
}

let treasure = Treasure::with_coordinates(2, 9) // [!code highlight]
    .label("oats".to_owned())
    .build();

assert_eq!(treasure.x, 2);
assert_eq!(treasure.y, 9);
assert_eq!(treasure.label.as_deref(), Some("oats"));
```

Here, the generated `with_coordinates` method has the following signature:

```rust ignore
impl Treasure {
    fn with_coordinates(x: u32, y: u32) -> TreasureBuilder { /**/ }
}
```

## Finishing function

Now let's say we need to know the person who claimed the `Treasure`. While describing the treasure using the current builder syntax we'd like the person who claimed it to specify their first name and last name at the end of the building process.

We can use a similar combination of the [top-level `#[builder(finish_fn = ...)]`](../reference/builder/top-level/finish-fn) and the [member-level `#[builder(finish_fn)]`](../reference/builder/member/finish-fn) attributes to do that.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
#[builder(
    start_fn = with_coordinates,
    finish_fn = claim  // [!code highlight]
)]
struct Treasure {
    #[builder(start_fn)]
    x: u32,

    #[builder(start_fn)]
    y: u32,

    #[builder(finish_fn)]          // [!code highlight]
    claimed_by_first_name: String, // [!code highlight]

    #[builder(finish_fn)]          // [!code highlight]
    claimed_by_last_name: String,  // [!code highlight]

    label: Option<String>,
}

let treasure = Treasure::with_coordinates(2, 9)
    .label("oats".to_owned())
    .claim("Lyra".to_owned(), "Heartstrings".to_owned()); // [!code highlight]

assert_eq!(treasure.x, 2);
assert_eq!(treasure.y, 9);
assert_eq!(treasure.label.as_deref(), Some("oats"));
assert_eq!(treasure.claimed_by_first_name, "Lyra");        // [!code highlight]
assert_eq!(treasure.claimed_by_last_name, "Heartstrings"); // [!code highlight]
```

## Into conversions

You may also combine these attributes with [`#[builder(into)]`](../reference/builder/member/into) or [`#[builder(on(..., into))]`](../reference/builder/top-level/on) to reduce the number of `to_owned()` calls a bit.

```rust
use bon::Builder;

#[derive(Builder)]
#[builder(
    start_fn = with_coordinates,
    finish_fn = claim // [!code focus]
)]
struct Treasure {
    #[builder(start_fn)]
    x: u32,

    #[builder(start_fn)]
    y: u32,

    #[builder(finish_fn, into)]    // [!code focus]
    claimed_by_first_name: String, // [!code focus]

    #[builder(finish_fn, into)]    // [!code focus]
    claimed_by_last_name: String,  // [!code focus]

    #[builder(into)]               // [!code focus]
    label: Option<String>,         // [!code focus]
}

let treasure = Treasure::with_coordinates(2, 9)
    .label("oats")                  // [!code focus]
    .claim("Lyra", "Heartstrings"); // [!code focus]
```

However, keep in mind that positional members (ones annotated with `#[builder(start_fn/finish_fn)]`) are always required to pass. There is no special treatment of the `Option` type for such members.

For example `#[builder(into)]` on a regular (named) member of the `Option<T>` type generates two setters:
- One that accepts `impl Into<T>`.
- The other that accepts `Option<impl Into<T>>`.

For positional members, the story is completely different because there are no separate setters generated for them. There is just a single starting or finishing function. So if you enable an into conversion for a positional member of the `Option<T>` type, it will be accepted as `impl Into<Option<T>>` in the starting or finishing function.

Also, the type pattern of the `#[builder(on(..., into))]` attribute matches the `Option<T>` fully. So, for example `on(String, into)` will not match the positional member of type `Option<String>`, but `on(Option<String>, into)` will.

::: tip

In general, it's not recommended to annotate optional members with `#[builder(start_fn/finish_fn)]` because you can't omit setting them using the positional function call syntax.

:::
