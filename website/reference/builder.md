---
outline: [2, 3]
---

# `#[derive(Builder)]` / `#[builder]`

Generate builders from structs via `#[derive(Builder)]`, free functions via `#[builder]`, and associated methods via `#[bon]` + `#[builder]`. They all use the same attributes syntax.

## Top-level attributes

These attributes are placed on top of a `struct` or `fn` declaration.

| Attribute | Short description
| -- | -- |
| [`builder_type`](./builder/top-level/builder-type) | Overrides the name, visibility and docs of the builder struct
| [`crate`](./builder/top-level/crate)               | Overrides the path to `bon` crate referenced in the generated code
| [`derive`](./builder/top-level/derive)             | Generates additional derives for the builder struct itself
| [`finish_fn`](./builder/top-level/finish-fn)       | Overrides the name, visibility and docs of the finishing function
| [`on`](./builder/top-level/on)                     | Applies the given builder attributes to all members matching a type pattern
| [`start_fn`](./builder/top-level/start-fn)         | Overrides the name, visibility and docs of the starting function

## Member attributes

These attributes are placed on a `struct` field or `fn` argument.

| Attribute | Short description
| -- | -- |
| [`default`](./builder/member/default)              | Makes the member optional with a default value
| [`finish_fn`](./builder/member/finish-fn)          | Makes the member a positional argument on the finishing function
| [`into`](./builder/member/into)                    | Changes the signature of the generated setters to accept `impl Into<T>`
| [`name`](./builder/member/name)                    | Overrides the name of the member used in the builder's API
| [`overwritable` 🔬](./builder/member/overwritable) | ??????? TODO: ADD DOCS ??????
| [`setters`](./builder/member/setters)              | ??????? TODO: ADD DOCS ??????
| [`skip`](./builder/member/skip)                    | Skips generating setters for the member
| [`start_fn`](./builder/member/start-fn)            | Makes the member a positional argument on the starting function
| [`transparent`](./builder/member/transparent)      | ??????? TODO: ADD DOCS ??????
| [`with`](./builder/member/with)                    | ??????? TODO: ADD DOCS ??????

## Examples

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
