# Compatibility

## Making a Required Member Optional

It's totally backwards compatible to make a required member optional by changing the type from `T` to `Option<T>` or by adding [`#[builder(default)]`](../../reference/builder/member/default) to it.

This is because both required and optional members have a setter that accepts `T` (not wrapped in an `Option`). The only change to the public API when making the required member optional is that a `maybe_`-prefixed setter is added to the builder. That new method accepts an `Option<T>`.

**Example:**

Suppose you have a function with a required argument, and there is existing code that sets that required argument using builder syntax.

```rust
use bon::builder;

#[builder]
fn get_page(password: &str) -> String {
    format!("Secret knowledge")
}

// Existing code that uses the builder API
let page = get_page()
    .password("I know the password!")
    .call();
assert_eq!(page, "Secret knowledge");
```

Then you change this function to take an `Option<T>`. This is totally fine, the old code that sets that parameter to `T` still compiles:

```rust
use bon::builder;

#[builder]
fn get_page(password: Option<&str>) -> String { // [!code highlight]
    format!("Secret knowledge")
}

// Existing code that uses the builder API (unchanged, still compiles)
let page = get_page()
    .password("I know the password!")
    .call();
assert_eq!(page, "Secret knowledge");

// Now this code can also pass `Option<T>`, including `None` to the function
// using the new `maybe_password` method.
get_page()
    .maybe_password(Some("password"))
    .call();
```

## Switching Between `Option<T>` and `#[builder(default)]`

Switching between `Option<T>` for the member type and `#[builder(default)]` on `T` is fully compatible. Nothing changes in the builder API when this happens.

**Example:**

```rust
use bon::builder;

#[builder]
fn example(filter: Option<String>) {}

example().maybe_filter(Some("filter".to_owned())).call();
```

This code can be changed to use `#[builder(default)]` and the call site will still compile:

```rust ignore
use bon::builder;

#[builder]
fn example(
    #[builder(default)]    // [!code ++]
    filter: Option<String> // [!code --]
    filter: String         // [!code ++]
) {}

example.maybe_filter(Some("filter".to_owned())).call();
```

## Marking Member as Unused with a Leading `_`

You may add `_` prefix to the member name to mark it as unused for the time being. The builder API won't change if you do that. Leading underscores are stripped from the setter names automatically.

**Example:**

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
struct Example {
    _name: String
}

Example::builder()
    .name("The setter is still called `name`".to_owned())
    .build();
```

```rust [Free function]
use bon::builder;

#[builder]
fn example(
    _name: String
) {}

example()
    .name("The setter is still called `name`".to_owned())
    .call();
```

```rust [Associated method]
use bon::{bon, builder};

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(_name: String) {}
}

Example::example()
    .name("The setter is still called `name`".to_owned())
    .call();
```

:::

## Switching Between `#[derive(Builder)]` and `#[builder]` on the `new()` Method

`#[derive(Builder)]` on a struct generates builder API that is fully compatible with placing `#[builder]` on the `new()` method with the signature similar to struct's fields.

This means, for example, it's preferable to place the `#[derive(Builder)]` attribute on top of your struct in most cases because it's convenient. However, if you need to have some custom logic during the construction of your type, you may simply create a `new()` method annotated with `#[builder]` where you can do anything you want to create an instance of your type.

To keep your struct's public API compatible with the time when `#[derive(Builder)]` was on the struct directly, the `new()` method must accept the same parameters as there were fields in the struct.

**Example:**

```rust ignore
use bon::Builder; // [!code --]
use bon::bon;     // [!code ++]

// Previously we used `#[derive(Builder)]` on the struct
#[derive(Builder)] // [!code --]
struct User {
    // But then we decided to change the internal representation
    // of the `id` field to use `String` instead of `u32`
    id: u32,                                                     // [!code --]
    id: String,                                                  // [!code ++]
    name: String,
}

// To preserve compatibility we need to define a `new()` method with `#[builder]`
// that still accepts `u32` for the `id` member.
#[bon]                                      // [!code ++]
impl User {                                 // [!code ++]
    #[builder]                              // [!code ++]
    fn new(id: u32, name: String) -> Self { // [!code ++]
        Self {                              // [!code ++]
            id: format!("u-{id}"),          // [!code ++]
            name,                           // [!code ++]
        }                                   // [!code ++]
    }                                       // [!code ++]
}                                           // [!code ++]

// The caller's code didn't change. It still uses `u32` for the `id` member.
let user = User::builder()
    // `id` is still accepted as a `u32` here
    .id(1)
    .name("Bon".to_owned())
    .build();
```

## Adding `#[builder]` to Existing Functions

Let's suppose you have existing code that defines functions with positional parameters in the public API. You'd like to change it to expose builder syntax instead, but you want to keep the old code compatible with the positional functions API.

In this case, you may use the top-level attribute `#[builder(start_fn)]` to keep both syntaxes available. See examples in the [docs for this attribute](../../reference/builder/top-level/start_fn#exposing-original-function).
