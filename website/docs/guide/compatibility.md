# Compatibility

## Changing required setter to optional

It's totally fine to change a required setter to an optional one by changing the type from `T` to `Option<T>` or by adding [`#[builder(default)]`](../reference/builder.md#default) to it.

This is because the generated setter will still accept `T` in its parameter. The only change to the public API is that a `maybe_`-prefixed setter is added to the builder.

**Example:**

Suppose you had a function with required parameters.

```rust
use bon::builder;

#[builder]
fn get_page(password: String) -> String {
    format!("Secret knowledge")
}

let page = get_page()
    .password("I know the password!")
    .call();
assert_eq!(page, "Secret knowledge");
```

Then you changed this function to take an `Option<T>`. This is totally fine, the old code that sets that parameter to `T` still compiles:

```rust
use bon::builder;

#[builder]
fn get_page(password: Option<String>) -> String { // [!code highlight]
    format!("Secret knowledge")
}

let page = get_page()
    .password("I know the password!")
    .call();
assert_eq!(page, "Secret knowledge");

// Now this code can also pass `Option<T>`, including `None` to the function
let password = Some("password");

get_page()
    .maybe_password(password)
    .call();
```

## Switching between `Option<T>` and `#[builder(default)]`

Switching between `Option<T>` for the struct field or function argument type and `#[builder(default)]` on `T` is fully compatible. Nothing changes in the builder API when this happens.

**Example:**

```rust
use bon::builder;

#[builder]
fn example(filter: Option<String>) {}

example().maybe_filter(Some("filter")).call();
```

This code can be changed to use `#[builder(default)]` and the call site still compiles:

```rust ignore
use bon::builder;

#[builder]
fn example(
    #[builder(default)]    // [!code ++]
    filter: Option<String> // [!code --]
    filter: String         // [!code ++]
) {}

example.maybe_filter(Some("filter")).call();
```


## Moving `#[builder]` from the struct the `new()` method

`#[builder]` on a struct generates builder API that is fully compatible with placing `#[builder]` on the `new()` method with the signature similar to struct's fields

This means, for example, it's preferable to place the `#[builder]` attribute on top of your struct in most cases because it's convenient. However, if you need to have some custom logic during the construction of your type, you may simply create a `new()` method annotated with `#[builder]` where you can do anything you want to create an instance of your type.

To keep type's public API compatible with the time when `#[builder]` was on the struct directly, the `new()` method must accept the same parameters as there were fields on the struct.

**Example:**

```rust
use bon::bon;

struct User {
    // Suppose we decided to change the internal representation // [!code highlight]
    // of the `id` field of the struct to use `String`          // [!code highlight]
    id: String,                                                 // [!code highlight]
    name: String,
}

#[bon] // [!code highlight]
impl User {
    #[builder] // [!code highlight]
    fn new(id: u32, name: String) -> Self {
        Self {
            id: format!("u-{id}"),
            name,
        }
    }
}

// This code still compiles since the API of the builder didn't change // [!code highlight]
let user = User::builder()
    // `id` is still accepted as a `u32` here
    .id(1)
    .name("Bon")
    .build();

assert_eq!(user.id, "u-1");
assert_eq!(user.name, "Bon");
```

## Adding #[builder] to existing code

If your existing code defines functions with positional parameters in its public API that you'd like to change to use builder syntax, but you want to keep the old code compatible with the positional functions API, then you may use `#[builder(expose_positional_fn)]` attribute to keep both syntaxes available. See [this attribute's docs](../reference/builder#expose-positional-fn) for details.
