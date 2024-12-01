---
outline: deep
---

## Top-Level Attributes

These attributes are placed on top of a `struct` or `fn` declaration.

| Attribute                                          | Short description                                                                                    |
| -------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| [`builder_type`](./builder/top-level/builder_type) | Overrides name, visibility and docs for the builder struct                                           |
| [`crate`](./builder/top-level/crate)               | Overrides path to `bon` crate referenced in the generated code                                       |
| [`derive`](./builder/top-level/derive)             | Generates additional derives for the builder struct itself                                           |
| [`finish_fn`](./builder/top-level/finish_fn)       | Overrides name, visibility and docs for the finishing function                                       |
| [`on`](./builder/top-level/on)                     | Applies member attributes to all members matching a type pattern                                     |
| [`start_fn`](./builder/top-level/start_fn)         | Overrides name, visibility and docs for the starting function                                        |
| [`state_mod`](./builder/top-level/state_mod)       | Overrides name, visibility and docs for the builder's [typestate API](../guide/typestate-api) module |

## Member Attributes

These attributes are placed on a `struct` field or `fn` argument.

| Attribute                                          | Short description                                                |
| -------------------------------------------------- | ---------------------------------------------------------------- |
| [`default`](./builder/member/default)              | Makes the member optional with a default value                   |
| [`field`](./builder/member/field)                  | Defines a private field on the builder without setters           |
| [`finish_fn`](./builder/member/finish_fn)          | Makes the member a positional argument on the finishing function |
| [`getter`](./builder/member/getter)                | Makes the member have getter method for `&T`                     |
| [`into`](./builder/member/into)                    | Changes the signature of the setters to accept `impl Into<T>`    |
| [`name`](./builder/member/name)                    | Overrides the name of the member used in the builder's API       |
| [`overwritable` 🔬](./builder/member/overwritable) | Allows calling setters for the same member repeatedly            |
| [`required`](./builder/member/required)            | Disables `Option<T>` special handling, makes the member required |
| [`setters`](./builder/member/setters)              | Overrides name, visibility and docs for setters                  |
| [`skip`](./builder/member/skip)                    | Skips generating setters for the member                          |
| [`start_fn`](./builder/member/start_fn)            | Makes the member a positional argument on the starting function  |
| [`with`](./builder/member/with)                    | Overrides setters' signature and applies a custom conversion     |

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

```rust [Function]
use bon::builder;

#[builder(finish_fn = finish)] // <-- this is a top-level attribute // [!code highlight]
fn example(
    #[builder(default)] // <-- this is a member attribute // [!code highlight]
    arg: u32
) { }
```

```rust [Method]
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
