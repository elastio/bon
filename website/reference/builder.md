---
outline: [2, 3]
---

# `#[derive(Builder)]` / `#[builder]`

You can generate a builder using three different kinds of syntax (struct, free function, associated method). They all share two common groups of attributes.

- [Top-level attributes](#top-level-attributes) - placed on a `struct` or `fn` declaration.
- [Member attributes](#member-attributes) - placed on a `struct` field or `fn` argument.

See examples. Make sure to click through the tabs:

:::code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
#[builder(finish_fn = finish)] // <-- this is a top-level attribute // [!code highlight]
struct Example {
    #[builder(default)] // <-- this is a member attribute // [!code highlight]
    field: u32
}
```

```rust [Free function]
use bon::builder;

#[builder(finish_fn = finish)] // <-- this is a top-level attribute // [!code highlight]
fn example(
    #[builder(default)] // <-- this is a member attribute // [!code highlight]
    arg: u32
) { }
```

```rust [Associated method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder(finish_fn = finish)] // <-- this is a top-level attribute // [!code highlight]
    fn example(
        #[builder(default)]  // <-- this is a member attribute // [!code highlight]
        arg: u32
    ) { }
}
```

---

Most of the attributes apply to all kinds of syntax. However, some of them are only available with structs or only with functions/methods, for example. The **"Applies to"** clause specifies the contexts where the attribute can be used.

## Top-level attributes

Click on the name of the attribute to view detailed docs.

| Attribute | Short description |
| -- | -- |
| [`builder_type`](./builder/top-level/builder-type) | Overrides the name, visibility and docs of the generated builder
| [`derive`](./builder/top-level/derive)             | Generates additional derives on the builder struct itself
| [`finish_fn`](./builder/top-level/finish-fn)       | Overrides the name, visibility and docs of the finishing function
| [`on`](./builder/top-level/on) | Applies the given builder attributes to all members, that match a type pattern
| [`start_fn`](./builder/top-level/start-fn) | Overrides the name, visibility and docs of the starting function

## Member attributes

### `expose_positional_fn`

**Applies to:** <Badge text="free functions"/> <Badge text="associated methods"/>

When generating builder code for functions the `#[builder]` macro hides the original function with positional parameters used to define the builder. That function is invoked inside of the builder's `call()` or `build()` method.

Usually you'd want the underlying positional function to be hidden to provide only the builder syntax to the callers. However, in some situations you may want to keep the positional function exposed along with the builder syntax for compatibility with old code that still uses the old positional function call syntax.

This attribute can take several forms.

-   Simple: `#[builder(expose_positional_fn = identifier)]`. Sets only the name of the positional function.
-   Verbose: `#[builder(expose_positional_fn(name = identifier, vis = "visibility"))]`.
    Allows setting both the name and the visibility of the positional function.
    Each key is optional. The `vis` must be specified as a string literal e.g. `"pub(crate)"`, `"pub"` or `""` (empty string means private visibility).

If `vis` parameter is not specified, then the visibility of the exposed positional function will be the same as specified on the function that the `#[builder]` was applied to.

**Example:**

::: code-group

```rust ignore [Free function]
use bon::builder;

#[builder(expose_positional_fn = example_positional)] // [!code highlight]
fn example(x: u32, y: u32) {}

// Positional function is now available under the given name  // [!code highlight]
example_positional(1, 2);                                     // [!code highlight]

// Builder syntax is also available (unchanged)
example()
    .x(1)
    .y(2)
    .call();
```

```rust ignore [Associated method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder(expose_positional_fn = example_positional)] // [!code highlight]
    fn example(x: u32, y: u32) {}
}

// Positional function is now available under the given name  // [!code highlight]
Example::example_positional(1, 2);                            // [!code highlight]

// Builder syntax is also available (unchanged)
Example::example()
    .x(1)
    .y(2)
    .call();
```

:::

#### `new` method special case

There are two conventional names in Rust ecosystem for constructors and builders:

-   `new` is used for a constructor method that uses positional parameters
-   `builder` is used for a method that returns a builder for a type

So when `#[builder]` is placed on a method called `new`, it'll generate a method called `builder` that starts the building process. This means there is already a default obvious name for the positional function that `expose_positional_fn` may use in this case if you don't specify any value for this attribute.

**Example:**

```rust ignore
use bon::bon;

struct Example {
    x: u32,
    y: u32,
}

#[bon]
impl Example {
    #[builder(expose_positional_fn)] // [!code highlight]
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

// Positional function is available under the name `new` // [!code highlight]
Example::new(1, 2);                                      // [!code highlight]

// Builder syntax is also available (unchanged)
Example::builder()
    .x(1)
    .y(2)
    .build();
```

This makes it possible to add builder syntax to your existing types that have the `new` method without breaking compatibility with old code. Old code can still use `T::new()` syntax, while new code can benefit from `T::builder()` syntax.
