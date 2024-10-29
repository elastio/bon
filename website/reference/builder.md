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

Most of the attributes apply both to `struct` and `fn` syntaxes, but there are exceptions. The **"Applies to"** clause at the top of the detailed docs clarifies this for every attribute.

## Top-level attributes

Click on the name of the attribute to view detailed docs.

| Attribute | Short description
| -- | -- |
| [`builder_type`](./builder/top-level/builder-type) | Overrides the name, visibility and docs of the builder struct
| [`derive`](./builder/top-level/derive)             | Generates additional derives on the builder struct itself
| [`finish_fn`](./builder/top-level/finish-fn)       | Overrides the name, visibility and docs of the finishing function
| [`on`](./builder/top-level/on)                     | Applies the given builder attributes to all members matching a type pattern
| [`start_fn`](./builder/top-level/start-fn)         | Overrides the name, visibility and docs of the starting function

## Member attributes

Click on the name of the attribute to view detailed docs.

| Attribute | Short description
| -- | -- |
| [`default`](./builder/member/default) | Makes the member optional with a default value
| [`finish_fn`](./builder/member/finish-fn) | Makes the member a positional argument on the finishing function
| [`into`](./builder/member/into) | Changes the signature of the generated setters to accept `impl Into<T>`
| [`name`](./builder/member/name) | Overrides the name of the member used in the builder's API
| [`skip`](./builder/member/skip) | Skips generating setters for the member
| [`start_fn`](./builder/member/start-fn) | Makes the member a positional argument on the starting function
