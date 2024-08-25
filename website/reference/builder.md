---
outline: [2, 3]
---

# `#[builder]`

## Top-Level Attributes

### `builder_type`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/>

Overrides the name of the generated builder struct.

The default naming pattern is the following:

| Type of item `#[builder]` is placed on | Naming pattern                                |
| -------------------------------------- | --------------------------------------------- |
| Struct                                 | `{StructName}Builder`                         |
| `StructName::new()` method             | `{StructName}Builder`                         |
| Free function                          | `{PascalCaseFunctionName}Builder`             |
| Associated method                      | `{SelfTypeName}{PascalCaseMethodName}Builder` |

The attribute expects the desired builder type identifier as its input.

**Example:**

::: code-group

```rust [Struct]
use bon::builder;

#[builder(builder_type = MyBuilder)] // [!code highlight]
struct Brush {}

let builder: MyBuilder = Brush::builder();
```

```rust [Free function]
use bon::builder;

#[builder(builder_type = MyBuilder)] // [!code highlight]
fn brush() {}

let builder: MyBuilder = brush();
```

```rust [Associated method]
use bon::bon;

struct Brush;

#[bon]
impl Brush {
    #[builder(builder_type = MyBuilder)] // [!code highlight]
    fn new() -> Self {
        Self
    }
}

let builder: MyBuilder = Brush::builder();
```

:::

You'll usually want to override the builder type name when you already have such a name in scope. For example, if you have a struct and a function with the same name annotated with `#[builder]`:

::: code-group

```rust compile_fail [Errored]
use bon::builder;

#[builder] // [!code error]
struct Brush {}

#[builder] // [!code error]
fn brush() {}

// `BrushBuilder` builder type name was generated for both
// the struct and the function. This is a compile error
let builder: BrushBuilder = Brush::builder();
let builder: BrushBuilder = brush();
```

```rust [Fixed]
use bon::builder;

#[builder(builder_type = MyBuilder)] // [!code highlight]
struct Brush {}

#[builder]
fn brush() {}

// Now builder types are named differently
let builder: MyBuilder = Brush::builder();
let builder: BrushBuilder = brush();
```

:::

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

```rust [Free function]
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

```rust [Associated method]
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

```rust
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
use bon::builder;

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
use bon::builder;

#[builder(start_fn = init)] // [!code highlight]
struct User {
    id: u32
}

User::init() // [!code highlight]
    .id(42)
    .build();
```

```rust [Verbose form]
use bon::builder;

// `User::init()` method will have `pub(crate)` visibility // [!code highlight]
// Use `vis = ""` to make it fully private instead         // [!code highlight]
#[builder(start_fn(name = init, vis = "pub(crate)"))]      // [!code highlight]
pub struct User {
    id: u32
}

User::init() // [!code highlight]
    .id(42)
    .build();
```

:::

### `on`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/>

Applies the given builder attributes to all members that match the selected type pattern. For example, you can automatically apply `#[builder(into)]` to all members of type `String` this way:

::: code-group

```rust [Struct]
use bon::builder;

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
use bon::builder;

#[builder(on(String, into))]
struct Example {
    name: String,
    description: Option<String>,

    #[builder(default)]
    alias: String
}

Example::builder()
    .name("Bon")
    // These members also matched the `String` type pattern,
    // so `#[builder(into)]` was applied to it
    .description("accepts an `impl Into<String>` here")
    .alias("builder")
    .build();
```

You can specify `on(...)` multiple times.

**Example:**

```rust
use bon::builder;
use std::path::PathBuf;

#[builder(on(String, into), on(PathBuf, into))]
struct Example {
    name: String,
    path: PathBuf,
    level: u32,
}

Example::builder()
    .name("accepts `impl Into<String>`")
    .path("accepts/impl/into/PathBuf")
    // This member doesn't match neither `String` nor `PathBuf`,
    // and thus #[builder(into)] was not applied to it
    .level(100)
    .build();
```



## Member-Level Attributes

### `default`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member optional and assigns a default value to it. There will be two setter methods generated for the member just like for [members of type `Option<T>`](../guide/optional-members). One setter accepts a value of type `T` (type of the member) and the other (with the `maybe_` prefix) accepts an `Option<T>`.

::: tip

Switching between `#[builder(default)]` and `Option<T>` is [compatible](../guide/compatibility#switching-between-optiont-and-builderdefault).

:::

The default value will be lazily computed inside of the [finishing function](#finish_fn) (i.e. `build()` or `call()`). It is computed only if the setter for the member wasn't called or `None` was passed to the `maybe_{member}()` setter.

The default value is computed based on the form of this attribute:

| Form                               | How default value is computed |
| ---------------------------------- | ----------------------------- |
| `#[builder(default)]`              | `Default::default()`          |
| `#[builder(default = expression)]` | `expression`                  |

The result of the `expression` will be converted into the target type using [`Into::into`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html) if [`#[builder(into)]`](#into) is enabled for the setter.

**Example:**

::: code-group

```rust [Struct field]
use bon::builder;

#[builder]
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

#### Compile errors

This attribute is incompatible with members of `Option` type, since `Option` already implies the default value of `None`.

### `into`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Changes the signature of the generated setters to accept [`impl Into<T>`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html), where `T` is the type of the member.

For [optional members](../guide/optional-members), the `maybe_{member}()` setter method will accept an `Option<impl Into<T>>` type instead of just `Option<T>`.

For members that use `#[builder(default = expression)]`, the `expression` will be converted with `Into::into`.

This parameter is often used with the `String` type, which allows you to pass `&str` into the setter without calling `.to_owned()` or `.to_string()` on it.

See the ["Into conversions"](../guide/patterns/into-conversions) page that shows the common patterns and antipatterns of `impl Into<T>`.

**Example:**

::: code-group

```rust [Struct field]
use bon::builder;

#[builder]
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

Overrdies the name for the setters generated for the member. This is most useful when `#[builder]` is placed on a struct where you'd like to use a different name for the field internally. For functions this attribute makes less sense since it's easy to just create a variable named differently `let new_name = param_name;`. However, this attribute is still supported for functions.

**Example:**

::: code-group

```rust [Struct field]
use bon::builder;

#[builder]
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

This can be used to give a name for the function arguments that use destructuring patterns,
although it's simpler to just destructure inside of the function body, which should be preferred over using this attribute.

**Example:**

::: code-group

```rust [Preferred destructuring in function body]
use bon::builder;

#[builder]
fn example(point: (u32, u32)) {
    let (x, y) = point;
}

example()
    .point((1, 2))
    .call();
```

```rust [Discouraged name attribute in destructuring]
use bon::builder;

#[builder]
fn example(
    #[builder(name = point)]
    (x, y): (u32, u32)
) {}

example()
    .point((1, 2))
    .call();
```

:::

### `skip`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Skips generating setters for the member. This hides the member from the generated builder API, so the caller can't set its value.

The value for the member will be computed based on the form of the attribute:

| Form                            | How value for the member is computed |
| ------------------------------- | ------------------------------------ |
| `#[builder(skip)]`              | `Default::default()`                 |
| `#[builder(skip = expression)]` | `expression`                         |

**Example:**

::: code-group

```rust [Struct field]
use bon::builder;

#[builder]
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

```rust [Free function argument]
use bon::builder;

#[builder]
fn greet_user(
    #[builder(skip)] // [!code highlight]
    level: u32,

    // Any complex expression is accepted // [!code highlight]
    #[builder(skip = "anon".to_owned())]  // [!code highlight]
    name: String,
) -> String {
    format!("Hello {name}! Your level is {level}")
}

let greeting = greet_user()
    // There are no `level`, and `name` setters generated // [!code highlight]
    .call();

assert_eq!(greeting, "Hello anon! Your level is 0");
```

```rust [Associated method argument]
use bon::bon;

struct User {
    level: u32,
    name: String,
}

#[bon]
impl User {
    #[builder]
    fn new(
        #[builder(skip)] // [!code highlight]
        level: u32,

        // Any complex expression is accepted // [!code highlight]
        #[builder(skip = "anon".to_owned())]  // [!code highlight]
        name: String,
    ) -> Self {
        Self { level, name }
    }
}

let user = User::builder()
    // There are no `level`, and `name` setters generated // [!code highlight]
    .build();

assert_eq!(user.level, 0);
assert_eq!(user.name, "anon");
```

:::

*[Member]: Struct field or a function argument
*[member]: Struct field or a function argument
*[members]: Struct fields or function arguments
