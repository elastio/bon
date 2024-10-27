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

::: tip Historical note

`#[derive(bon::Builder)]` syntax appeared with the version `2.2` of `bon`. The older versions of `bon` (i.e. `<= 2.1`) supported only `#[bon::builder]` syntax with structs, but that syntax was deprecated in favor of the `derive` syntax for various reasons described in the [2.2 release blog post](../blog/bon-builder-v2-2-release).

:::

## Top-level attributes

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

### `finish_fn`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/>

This attribute allows overriding the name of the generated builder's method that finishes the building process.

**Example:**

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
#[builder(finish_fn = assemble)] // [!code highlight]
struct Article {
    id: u32
}

let article = Article::builder()
    .id(42)
    .assemble(); // [!code highlight]

assert_eq!(article.id, 42);
```

```rust [Free function]
use bon::builder;

#[builder(finish_fn = send)] // [!code highlight]
fn get_article(id: u32) -> String {
    format!("Some article with id {id}")
}

let response = get_article()
    .id(42)
    .send(); // [!code highlight]

assert_eq!(response, "Some article with id 42");
```

```rust [Associated method]
use bon::bon;

struct ArticlesClient;

#[bon]
impl ArticlesClient {
    #[builder(finish_fn = send)] // [!code highlight]
    fn get_article(&self, id: u32) -> String {
        format!("Some article with id {id}")
    }
}

let response = ArticlesClient
    .get_article()
    .id(42)
    .send(); // [!code highlight]

assert_eq!(response, "Some article with id 42");
```

:::

### `on`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/>

Applies the given builder attributes to all members that match the selected type pattern. For example, you can automatically apply `#[builder(into)]` to all members of type `String` this way:

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
#[builder(on(String, into))]
struct Example {
    id: String,
    name: String,
    level: u32,
}

Example::builder()
    // `id` and `name` accept `impl Into<String>` because   // [!code highlight]
    // `on` automatically added `#[builder(into)]` for them // [!code highlight]
    .id("e-1111")
    .name("Bon")
    // `u32` doesn't match the `String` type pattern, // [!code highlight]
    // so `#[builder(into)]` was not applied to it    // [!code highlight]
    .level(100)
    .build();
```

```rust [Free function]
use bon::builder;

#[builder(on(String, into))]
fn example(
    id: String,
    name: String,
    level: u32,
) {}

example()
    // `id` and `name` accept `impl Into<String>` because   // [!code highlight]
    // `on` automatically added `#[builder(into)]` for them // [!code highlight]
    .id("e-1111")
    .name("Bon")
    // `u32` doesn't match the `String` type pattern, // [!code highlight]
    // so `#[builder(into)]` was not applied to it    // [!code highlight]
    .level(100)
    .call();
```

```rust [Associated method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder(on(String, into))]
    fn example(
        id: String,
        name: String,
        level: u32,
    ) {}
}

Example::example()
    // `id` and `name` accept `impl Into<String>` because   // [!code highlight]
    // `on` automatically added `#[builder(into)]` for them // [!code highlight]
    .id("e-1111")
    .name("Bon")
    // `u32` doesn't match the `String` type pattern, // [!code highlight]
    // so `#[builder(into)]` was not applied to it    // [!code highlight]
    .level(100)
    .call();
```

:::

This attribute must be of form `on(type_pattern, attributes)`.

- `type_pattern` - type that will be compared with the types of the members. The types are compared textually. For example, `String` doesn't match `std::string::String`. You can use `_` to mark parts of the type to ignore when matching. For example, `Vec<_>` matches `Vec<u32>` or `Vec<String>`. Lifetimes are ignored during matching.

- `attributes` - for now, the only attribute supported in the `attributes` position is [`into`](#into). It sets `#[builder(into)]` for members that match the `type_pattern`.

If you want to apply the `attributes` to all members, you can use the `_` type pattern that matches any type. For example, `#[builder(on(_, into))]`.

For optional members the underlying type is matched ignoring the `Option` wrapper.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
#[builder(on(String, into))]
struct Example {
    name: String,
    description: Option<String>,

    #[builder(default)]
    alias: String
}

Example::builder()
    .name("Bon")
    // These members also match the `String` type pattern,
    // so `#[builder(into)]` was applied to them
    .description("accepts an `impl Into<String>` here")
    .alias("builder")
    .build();
```

You can specify `on(...)` multiple times.

**Example:**

```rust
use bon::Builder;
use std::path::PathBuf;

#[derive(Builder)]
#[builder(on(String, into), on(PathBuf, into))]
struct Example {
    name: String,
    path: PathBuf,
    level: u32,
}

Example::builder()
    .name("accepts `impl Into<String>`")
    .path("accepts/impl/into/PathBuf")
    // This member doesn't match either `String` or `PathBuf`,
    // and thus #[builder(into)] was not applied to it
    .level(100)
    .build();
```

### `start_fn`

**Applies to:** <Badge text="structs"/>

Overrides the name and visibility of the associated method that starts the building process, i.e. returns the builder for the struct.

The default name for this method is `builder`, and the default visibility is the same as the visibility of the struct itself.

This attribute can take several forms.

-   Simple: `#[builder(start_fn = identifier)]`. Overrides only the name of the "start" method.
-   Verbose: `#[builder(start_fn(name = identifier, vis = "visibility"))]`.
    Allows overriding both the name and the visibility of the "start" method.
    Each key is optional. The `vis` must be specified as a string literal e.g. `"pub(crate)"`, `"pub"` or `""` (empty string means private visibility).

**Example:**

::: code-group

```rust [Simple form]
use bon::Builder;

#[derive(Builder)]
#[builder(start_fn = init)] // [!code highlight]
struct User {
    id: u32
}

User::init() // [!code highlight]
    .id(42)
    .build();
```

```rust [Verbose form]
use bon::Builder;

// `User::init()` method will have `pub(crate)` visibility // [!code highlight]
// Use `vis = ""` to make it fully private instead         // [!code highlight]
#[derive(Builder)]
#[builder(start_fn(name = init, vis = "pub(crate)"))]      // [!code highlight]
pub struct User {
    id: u32
}

User::init() // [!code highlight]
    .id(42)
    .build();
```

:::

## Member attributes

### `default`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member optional and assigns a default value to it. There will be two setter methods generated for the member just like for [members of type `Option<T>`](../guide/optional-members). One setter accepts a value of type `T` (type of the member) and the other (with the `maybe_` prefix) accepts an `Option<T>`.

::: tip

Switching between `#[builder(default)]` and `Option<T>` is [compatible](../guide/compatibility#switching-between-option-t-and-builder-default).

:::

The default value will be lazily computed inside of the [finishing function](#finish-fn) (i.e. `build()` or `call()`). It is computed only if the setter for the member wasn't called or `None` was passed to the `maybe_{member}()` setter.

The default value is computed based on the form of this attribute:

| Form                               | How default value is computed |
| ---------------------------------- | ----------------------------- |
| `#[builder(default)]`              | `Default::default()`          |
| `#[builder(default = expression)]` | `expression`                  |

The result of the `expression` will be converted into the target type using [`Into::into`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html) if [`#[builder(into)]`](#into) is enabled for the setter.

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
struct User {
    #[builder(default)] // [!code highlight]
    level: u32,

    // The expression of type `&'static str` is automatically             // [!code highlight]
    // converted to `String` here via `Into` thanks to `#[builder(into)]. // [!code highlight]
    #[builder(into, default = "anon")]                                    // [!code highlight]
    name: String,

    // Any complex expression is accepted   // [!code highlight]
    #[builder(default = bon::vec!["read"])] // [!code highlight]
    permissions: Vec<String>,
}

let user = User::builder().build();

assert_eq!(user.level, 0);
assert_eq!(user.name, "anon");
assert_eq!(user.permissions, ["read"]);
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn greet_user(
    #[builder(default)] // [!code highlight]
    level: u32,

    // The expression of type `&'static str` is automatically             // [!code highlight]
    // converted to `String` here via `Into` thanks to `#[builder(into)]. // [!code highlight]
    #[builder(into, default = "anon")]                                    // [!code highlight]
    name: String,

    // Any complex expression is accepted   // [!code highlight]
    #[builder(default = bon::vec!["read"])] // [!code highlight]
    permissions: Vec<String>,
) -> String {
    format!("Hello {name}! Your level is {level}, permissions: {permissions:?}")
}

let greeting = greet_user().call();

assert_eq!(greeting, "Hello anon! Your level is 0, permissions: [\"read\"]");
```

```rust [Associated method argument]
use bon::bon;

struct User {
    level: u32,
    name: String,
    permissions: Vec<String>,
}

#[bon]
impl User {
    #[builder]
    fn new(
        #[builder(default)] // [!code highlight]
        level: u32,

        // The expression of type `&'static str` is automatically             // [!code highlight]
        // converted to `String` here via `Into` thanks to `#[builder(into)]. // [!code highlight]
        #[builder(into, default = "anon")]                                    // [!code highlight]
        name: String,

        // Any complex expression is accepted   // [!code highlight]
        #[builder(default = bon::vec!["read"])] // [!code highlight]
        permissions: Vec<String>,
    ) -> Self {
        Self { level, name, permissions }
    }
}

let user = User::builder().build();

assert_eq!(user.name, "anon");
assert_eq!(user.level, 0);
assert_eq!(user.permissions, ["read"]);
```

:::

You can also use the values of other members by referencing their names in the `default` expression. All members are initialized in the order of their declaration. It means only those members that are declared earlier (higher) in the code are available to the `default` expression.

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
struct Example {
    member_1: u32,

    // Note that here we don't have access to `member_3`
    // because it's declared (and thus initialized) later
    #[builder(default = 2 * member_1)]
    member_2: u32,

    #[builder(default = member_2 + member_1)]
    member_3: u32,
}

let example = Example::builder()
    .member_1(3)
    .build();

assert_eq!(example.member_1, 3);
assert_eq!(example.member_2, 6);
assert_eq!(example.member_3, 9);
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn example(
    member_1: u32,

    // Note that here we don't have access to `member_3`
    // because it's declared (and thus initialized) later
    #[builder(default = 2 * member_1)]
    member_2: u32,

    #[builder(default = member_2 + member_1)]
    member_3: u32,
) -> (u32, u32, u32) {
    (member_1, member_2, member_3)
}

let example = example()
    .member_1(3)
    .call();

assert_eq!(example, (3, 6, 9));
```

```rust [Associated method argument]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        member_1: u32,

        // Note that here we don't have access to `member_3`
        // because it's declared (and thus initialized) later
        #[builder(default = 2 * member_1)]
        member_2: u32,

        #[builder(default = member_2 + member_1)]
        member_3: u32,
    ) -> (u32, u32, u32) {
        (member_1, member_2, member_3)
    }
}

let example = Example::example()
    .member_1(3)
    .call();

assert_eq!(example, (3, 6, 9));
```

:::

#### Caveats

The `self` parameter in associated methods is not available to the `default` expression. If you need the `self` context for your defaulting logic, then set your member's type to `Option<T>` and handle the defaulting in the function's body manually.

#### Compile errors

This attribute is incompatible with members of `Option` type, since `Option` already implies the default value of `None`.

### `finish_fn`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member a positional argument on the finishing function that consumes the builder and returns the resulting object (for struct syntax) or performs the requested action (for function/method syntax).

The ordering of members annotated with `#[builder(finish_fn)]` matters! They will appear in the same order relative to each other in the finishing function signature. They must also be declared at the top of the members list strictly after members annotated with [`#[builder(start_fn)]`](#start-fn-1) (if any).

This ensures a consistent initialization order, and it makes these members available for expressions in `#[builder(default/skip = ...)]` for regular members that follow them.

::: tip

Don't confuse this with the top-level [`#[builder(finish_fn = ...)]`](#finish-fn) attribute that can be used to configure the name and visibility of the finishing function. You'll likely want to use it in combination with this member-level attribute to define a better name for the finishing function.

:::

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
// Top-level attribute to give a better name for the finishing function // [!code highlight]
#[builder(finish_fn = sign)]                                            // [!code highlight]
struct Message {
    // Member-level attribute to mark the member as a parameter of `sign()` // [!code highlight]
    #[builder(finish_fn)] // [!code highlight]
    author_first_name: String,

    #[builder(finish_fn)] // [!code highlight]
    author_last_name: String,

    payload: String,
}

let message = Message::builder()
    .payload("Bon is great! Give it a ⭐".to_owned())
    .sign("Sweetie".to_owned(), "Drops".to_owned());

assert_eq!(message.payload, "Bon is great! Give it a ⭐");
assert_eq!(message.author_first_name, "Sweetie");
assert_eq!(message.author_last_name, "Drops");
```

```rust [Free function]
use bon::builder;

// Top-level attribute to give a better name for the finishing function // [!code highlight]
#[builder(finish_fn = send)]                                            // [!code highlight]
fn message(
    // Member-level attribute to mark the member as a parameter of `sign()` // [!code highlight]
    #[builder(finish_fn)] // [!code highlight]
    receiver_first_name: String,

    #[builder(finish_fn)] // [!code highlight]
    receiver_last_name: String,

    payload: String,
) {}

message()
    .payload("Bon is great! Give it a ⭐".to_owned())
    .send("Sweetie".to_owned(), "Drops".to_owned());
```

```rust [Associated method]
use bon::bon;

struct Chat {}

#[bon]
impl Chat {
    // Top-level attribute to give a better name for the finishing function // [!code highlight]
    #[builder(finish_fn = send)]                                            // [!code highlight]
    fn message(
        &self,

        // Member-level attribute to mark the member as a parameter of `sign()` // [!code highlight]
        #[builder(finish_fn)] // [!code highlight]
        receiver_first_name: String,

        #[builder(finish_fn)] // [!code highlight]
        receiver_last_name: String,

        payload: String,
    ) {}
}

let chat = Chat {};

chat.message()
    .payload("Bon is great! Give it a ⭐".to_owned())
    .send("Sweetie".to_owned(), "Drops".to_owned());
```

:::

You can also combine this attribute with [`#[builder(into)]`](#into) or [`#[builder(on(..., into))]`](#on) to add an into conversion for the parameter.

Importantly, `Into` conversions for such members work slightly differently from the regular (named) members in regard to the `Option` type. The `Option` type gives no additional meaning to the member annotated with `#[builder(finish_fn)]`. Thus, it is matched by the type pattern of `on(..., into)` and wrapped with `impl Into<Option<T>>` as any other type.

::: tip

In general, it's not recommended to annotate optional members with `#[builder(finish_fn)]` because you can't omit setting them using the positional function call syntax.

:::

### `into`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Changes the signature of the generated setters to accept [`impl Into<T>`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html), where `T` is the type of the member.

For [optional members](../guide/optional-members), the `maybe_{member}()` setter method will accept an `Option<impl Into<T>>` type instead of just `Option<T>`.

For members that use `#[builder(default = expression)]`, the `expression` will be converted with `Into::into`.

This parameter is often used with the `String` type, which allows you to pass `&str` into the setter without calling `.to_owned()` or `.to_string()` on it.

See the ["Into Conversions In-Depth"](../guide/patterns/into-conversions-in-depth) page that shows the common patterns and antipatterns of `impl Into<T>`.

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(into)] // [!code highlight]
    name: String,

    #[builder(into)] // [!code highlight]
    description: Option<String>,

    // The value passed to `default = ...` is converted with `into` as well // [!code highlight]
    #[builder(into, default = "anon")]                                      // [!code highlight]
    group: String
}

Example::builder()
    // We can pass `&str` because the setters accept `impl Into<String>`      // [!code highlight]
    .name("Bon")                                                              // [!code highlight]
    .description("Awesome crate 🐱. Consider giving it a star on Github ⭐") // [!code highlight]
    // We can pass `Option<&str>` to `maybe_` methods because they accept     // [!code highlight]
    // `Option<impl Into<String>>`                                            // [!code highlight]
    .maybe_group(Some("Favourites"))                                          // [!code highlight]
    .build();
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn example(
    #[builder(into)] // [!code highlight]
    name: String,

    #[builder(into)] // [!code highlight]
    description: Option<String>,

    // The value passed to `default = ...` is converted with `into` as well // [!code highlight]
    #[builder(into, default = "anon")]                                      // [!code highlight]
    group: String
) {}

example()
    // We can pass `&str` because the setters accept `impl Into<String>`      // [!code highlight]
    .name("Bon")                                                              // [!code highlight]
    .description("Awesome crate 🐱. Consider giving it a star on Github ⭐") // [!code highlight]
    // We can pass `Option<&str>` to `maybe_` methods because they accept     // [!code highlight]
    // `Option<impl Into<String>>`                                            // [!code highlight]
    .maybe_group(Some("Favourites"))                                          // [!code highlight]
    .call();
```

```rust [Associated method argument]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        #[builder(into)] // [!code highlight]
        name: String,

        #[builder(into)] // [!code highlight]
        description: Option<String>,

        // The value passed to `default = ...` is converted with `into` as well // [!code highlight]
        #[builder(into, default = "anon")]                                      // [!code highlight]
        group: String
    ) {}
}

Example::example()
    // We can pass `&str` because the setters accept `impl Into<String>`      // [!code highlight]
    .name("Bon")                                                              // [!code highlight]
    .description("Awesome crate 🐱. Consider giving it a star on Github ⭐") // [!code highlight]
    // We can pass `Option<&str>` to `maybe_` methods because they accept     // [!code highlight]
    // `Option<impl Into<String>>`                                            // [!code highlight]
    .maybe_group(Some("Favourites"))                                          // [!code highlight]
    .call();
```

:::

### `name`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Overrides the name of the member in the builder's setters and type state. This is most useful when with struct syntax (`#[derive(Builder)]`) where you'd like to use a different name for the field internally. For functions this attribute makes less sense since it's easy to just create a variable named differently `let new_name = param_name;`. However, this attribute is still supported on function arguments.

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
struct Player {
    #[builder(name = rank)] // [!code highlight]
    level: u32
}

Player::builder()
    .rank(10) // [!code highlight]
    .build();
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn player(
    #[builder(name = rank)] // [!code highlight]
    level: u32
) {}

player()
    .rank(10) // [!code highlight]
    .call();
```

```rust [Associated method argument]
use bon::bon;

struct Player {
    level: u32,
}

#[bon]
impl Player {
    #[builder]
    fn new(
        #[builder(name = rank)] // [!code highlight]
        level: u32
    ) -> Self {
        Self { level }
    }
}

Player::builder()
    .rank(10) // [!code highlight]
    .build();
```

:::

### `skip`

**Applies to:** <Badge type="warning" text="struct fields"/>

Skips generating setters for the member. This hides the member from the generated builder API, so the caller can't set its value.

The value for the member will be computed based on the form of the attribute:

| Form                            | How value for the member is computed |
| ------------------------------- | ------------------------------------ |
| `#[builder(skip)]`              | `Default::default()`                 |
| `#[builder(skip = expression)]` | `expression`                         |

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
struct User {
    #[builder(skip)] // [!code highlight]
    level: u32,

    // Any complex expression is accepted // [!code highlight]
    #[builder(skip = "anon".to_owned())]  // [!code highlight]
    name: String,
}

let user = User::builder()
    // There are no `level`, and `name` setters generated // [!code highlight]
    .build();

assert_eq!(user.level, 0);
assert_eq!(user.name, "anon");
```

You can also use the values of other members by referencing their names in the `skip` expression. All members are initialized in the order of their declaration. It means only those members that are declared earlier (higher) in the code are available to the `skip` expression.

**Example:**

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    member_1: u32,

    // Note that here we don't have access to `member_3`
    // because it's declared (and thus initialized) later
    #[builder(skip = 2 * member_1)]
    member_2: u32,

    #[builder(skip = member_2 + member_1)]
    member_3: u32,
}

let example = Example::builder()
    .member_1(3)
    .build();

assert_eq!(example.member_1, 3);
assert_eq!(example.member_2, 6);
assert_eq!(example.member_3, 9);
```

This attribute is not supported with free function arguments or associated method arguments because it's simply unnecessary there and can easier be expressed with local variables.

### `start_fn`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member a positional argument on the starting function that creates the builder.

The ordering of members annotated with `#[builder(start_fn)]` matters! They will appear in the same order relative to each other in the starting function signature. They must also be declared at the top of the members' list.

This ensures a consistent initialization order, and it makes these members available for expressions in `#[builder(default/skip = ...)]` for regular members that follow them.

::: tip

Don't confuse this with the top-level [`#[builder(start_fn = ...)]`](#start-fn) attribute that can be used to configure the name and visibility of the starting function. You'll likely want to use it in combination with this member-level attribute to define a better name for the starting function.

:::

**Example:**

::: code-group

```rust [Struct field]
use bon::Builder;

#[derive(Builder)]
// Top-level attribute to give a better name for the starting function // [!code highlight]
#[builder(start_fn = with_coordinates)]                                // [!code highlight]
struct Treasure {
    // Member-level attribute to mark the member as // [!code highlight]
    // a parameter of `with_coordinates()`          // [!code highlight]
    #[builder(start_fn)]                            // [!code highlight]
    x: u32,

    #[builder(start_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
}

let treasure = Treasure::with_coordinates(2, 9) // [!code highlight]
    .label("knowledge".to_owned())
    .build();

assert_eq!(treasure.x, 2);
assert_eq!(treasure.y, 9);
assert_eq!(treasure.label.as_deref(), Some("knowledge"));
```

```rust [Free function]
use bon::builder;

#[builder]
fn mark_treasure_at(
    #[builder(start_fn)] // [!code highlight]
    x: u32,

    #[builder(start_fn)] // [!code highlight]
    y: u32,

    label: Option<String>,
) {}

mark_treasure_at(2, 9)
    .label("knowledge".to_owned())
    .call();
```

```rust [Associated method]
use bon::bon;

struct Navigator {}

#[bon]
impl Navigator {
    #[builder]
    fn mark_treasure_at(
        &self,

        #[builder(start_fn)] // [!code highlight]
        x: u32,

        #[builder(start_fn)] // [!code highlight]
        y: u32,

        label: String,
    ) {}
}

let navigator = Navigator {};

navigator
    .mark_treasure_at(2, 9)
    .label("knowledge".to_owned())
    .call();
```

:::

You can also combine this attribute with [`#[builder(into)]`](#into) or [`#[builder(on(..., into))]`](#on) to add an into conversion for the parameter.

Importantly, `Into` conversions for such members work slightly differently from the regular (named) members in regard to the `Option` type. The `Option` type gives no additional meaning to the member annotated with `#[builder(start_fn)]`. Thus, it is matched by the type pattern of `on(..., into)` and wrapped with `impl Into<Option<T>>` as any other type.

::: tip

In general, it's not recommended to annotate optional members with `#[builder(start_fn)]` because you can't omit setting them using the positional function call syntax.

:::
