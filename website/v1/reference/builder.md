---
outline: [2, 3]
---

# `#[builder]`

## Top-level attributes

### `builder_type`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/>

Overrides the name of the generated builder struct.

The default naming pattern is the following:

| Type of item `#[builder]` is placed on | Naming pattern                                |
| -------------------------------------- | --------------------------------------------- |
| Struct                                 | `{StructName}Builder`                         |
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
- Simple: `#[builder(expose_positional_fn = identifier)]`. Sets only the name of the positional function.
- Verbose: `#[builder(expose_positional_fn(name = identifier, vis = "visibility"))]`.
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
- `new` is used for a constructor method that uses positional parameters
- `builder` is used for a method that returns a builder for a type

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
- Simple: `#[builder(start_fn = identifier)]`. Overrides only the name of the "start" method.
- Verbose: `#[builder(start_fn(name = identifier, vis = "visibility"))]`.
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

## Member-level attributes

### `default`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member optional. This means setters will be generated as if the type of the member was wrapped in an `Option`. In fact, this property is guaranteed. See [API compatibility](../guide/compatibility#switching-between-option-t-and-builder-default) for details.

If no setter for the member is called or `None` is passed, then the default value will be computed based on the form of this attribute:

Form                               | How default value is computed
-----------------------------------|----------------------------------------------------------------
`#[builder(default)]`              | `Default::default()`
`#[builder(default = expression)]` | `expression`

The result of the `expression` will automatically be converted into the target type if `Into` conversion is enabled for this setter i.e. the type satisfies [automatic `Into` conversion qualification rules], or there is a [`#[builder(into)]`](#into) override.

The default value will be lazily computed *only if needed* inside of the [finishing function](#finish-fn) (i.e. `build()` or `call()`).

**Example:**

::: code-group

```rust [Struct field]
use bon::builder;

#[builder]
struct User {
    #[builder(default)] // [!code highlight]
    level: u32,

    // The expression of type `&'static str` is automatically // [!code highlight]
    // converted to `String` here via `Into`.                 // [!code highlight]
    #[builder(default = "anon")]                              // [!code highlight]
    name: String,

    // Any complex expression is accepted   // [!code highlight]
    #[builder(default = bon::vec!["read"])] // [!code highlight]
    permissions: Vec<String>,
}

let user = User::builder().build();

assert_eq!(user.name, "anon");
assert_eq!(user.level, 0);
assert_eq!(user.permissions, ["read"]);
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn greet_user(
    #[builder(default)] // [!code highlight]
    level: u32,

    // The expression of type `&'static str` is automatically // [!code highlight]
    // converted to `String` here via `Into`.                 // [!code highlight]
    #[builder(default = "anon")]                              // [!code highlight]
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

        // The expression of type `&'static str` is automatically // [!code highlight]
        // converted to `String` here via `Into`.                 // [!code highlight]
        #[builder(default = "anon")]                              // [!code highlight]
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

Forces an `impl Into` conversion to be enabled or disabled in the generated setter methods. Use this to force-override the decision made by [automatic `Into` conversion qualification rules].

This parameter can be specified in one of the following ways:

| Form                       | Behavior                                                 |
| -------------------------- | -------------------------------------------------------- |
| `#[builder(into)]`         | Forcefully **enables** `impl Into` in the setter method  |
| `#[builder(into = false)]` | Forcefully **disables** `impl Into` in the setter method |

**Example:**

::: code-group

```rust [Struct field]
use bon::builder;
use std::num::NonZeroU32;

#[builder]
struct Example {
    // `u32` isn't qualified for an `Into` conversion by default        // [!code highlight]
    // because it's a primitive type. This attribute force-enables it.  // [!code highlight]
    #[builder(into)]                                                    // [!code highlight]
    force_enabled_into: u32,

    // `String` is qualified for `Into` conversion by default              // [!code highlight]
    // because it's a simple type path. This attribute force-disables it.  // [!code highlight]
    #[builder(into = false)]                                               // [!code highlight]
    force_disabled_into: String,
}

let non_zero_u32 = NonZeroU32::new(1).unwrap();

Example::builder()
    // setter accepts `impl Into<u32>`                        // [!code highlight]
    .force_enabled_into(non_zero_u32)                         // [!code highlight]
    // setter accepts `String` instead of `impl Into<String>` // [!code highlight]
    .force_disabled_into("".to_owned())                       // [!code highlight]
    .build();
```

```rust [Free function argument]
use bon::builder;
use std::num::NonZeroU32;

#[builder]
fn example(
    // `u32` isn't qualified for an `Into` conversion by default        // [!code highlight]
    // because it's a primitive type. This attribute force-enables it.  // [!code highlight]
    #[builder(into)]                                                    // [!code highlight]
    force_enabled_into: u32,

    // `String` is qualified for `Into` conversion by default              // [!code highlight]
    // because it's a simple type path. This attribute force-disables it.  // [!code highlight]
    #[builder(into = false)]                                               // [!code highlight]
    force_disabled_into: String,
) {}

let non_zero_u32 = NonZeroU32::new(1).unwrap();

example()
    // setter accepts `impl Into<u32>`                        // [!code highlight]
    .force_enabled_into(non_zero_u32)                         // [!code highlight]
    // setter accepts `String` instead of `impl Into<String>` // [!code highlight]
    .force_disabled_into("".to_owned())                       // [!code highlight]
    .call();
```

```rust [Associated method argument]
use bon::bon;
use std::num::NonZeroU32;

struct Example;

#[bon]
impl Example {
    #[builder]
    fn example(
        // `u32` isn't qualified for an `Into` conversion by default        // [!code highlight]
        // because it's a primitive type. This attribute force-enables it.  // [!code highlight]
        #[builder(into)]                                                    // [!code highlight]
        force_enabled_into: u32,

        // `String` is qualified for `Into` conversion by default              // [!code highlight]
        // because it's a simple type path. This attribute force-disables it.  // [!code highlight]
        #[builder(into = false)]                                               // [!code highlight]
        force_disabled_into: String,
    ) {}
}

let non_zero_u32 = NonZeroU32::new(1).unwrap();

Example::example()
    // setter accepts `impl Into<u32>`                        // [!code highlight]
    .force_enabled_into(non_zero_u32)                         // [!code highlight]
    // setter accepts `String` instead of `impl Into<String>` // [!code highlight]
    .force_disabled_into("".to_owned())                       // [!code highlight]
    .call();
```

:::

#### Compile errors

If the placement of this attribute wouldn't override the default behavior, a compile error will be generated requesting the removal of a redundant attribute.

For example, in the following code `String` already qualifies for an `Into` conversion according to the [automatic `Into` conversion qualification rules].

```rust compile_fail
use bon::builder;

#[builder]
struct Example {
    // Compile error: "This attribute is redundant and can be removed." // [!code error]
    #[builder(into)] // [!code error]
    string: String
}
```

### `name`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Overrides the name of the member in the builder's setters and type state. This is most useful when `#[derive(Builder)]` is placed on a struct where you'd like to use a different name for the field internally. For functions this attribute makes less sense since it's easy to just create a variable named differently `let new_name = param_name;`. However, this attribute is still supported for functions.

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

### `skip`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Skips generating setters for the member. This hides the member from the generated builder API, so the caller can't set it's value.

The value for the member will be computed based on the form of the attribute specified below.

Form                            | How value for the member is computed
--------------------------------|----------------------------------------------------------------
`#[builder(skip)]`              | `Default::default()`
`#[builder(skip = expression)]` | `expression`

The result of the `expression` will automatically be converted into the target type if the type satisfies [automatic `Into` conversion qualification rules].

**Example:**

::: code-group

```rust [Struct field]
use bon::builder;

#[builder]
struct User {
    #[builder(skip)] // [!code highlight]
    level: u32,

    // The expression of type `&'static str` is automatically // [!code highlight]
    // converted to `String` here via `Into`.                 // [!code highlight]
    #[builder(skip = "anon")]                                 // [!code highlight]
    name: String,

    // Any complex expression is accepted // [!code highlight]
    #[builder(skip = bon::vec!["read"])]  // [!code highlight]
    permissions: Vec<String>,
}

let user = User::builder()
    // There are no `level`, `name`, and `permissions` setters generated // [!code highlight]
    .build();

assert_eq!(user.name, "anon");
assert_eq!(user.level, 0);
assert_eq!(user.permissions, ["read"]);
```

```rust [Free function argument]
use bon::builder;

#[builder]
fn greet_user(
    #[builder(skip)] // [!code highlight]
    level: u32,

    // The expression of type `&'static str` is automatically // [!code highlight]
    // converted to `String` here via `Into`.                 // [!code highlight]
    #[builder(skip = "anon")]                                 // [!code highlight]
    name: String,

    // Any complex expression is accepted // [!code highlight]
    #[builder(skip = bon::vec!["read"])]  // [!code highlight]
    permissions: Vec<String>,
) -> String {
    format!("Hello {name}! Your level is {level}, permissions: {permissions:?}")
}

let greeting = greet_user()
    // There are no `level`, `name`, and `permissions` setters generated // [!code highlight]
    .call();

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
        #[builder(skip)] // [!code highlight]
        level: u32,

        // The expression of type `&'static str` is automatically // [!code highlight]
        // converted to `String` here via `Into`.                 // [!code highlight]
        #[builder(skip = "anon")]                                 // [!code highlight]
        name: String,

        // Any complex expression is accepted // [!code highlight]
        #[builder(skip = bon::vec!["read"])]  // [!code highlight]
        permissions: Vec<String>,
    ) -> Self {
        Self { level, name, permissions }
    }
}

let user = User::builder()
    // There are no `level`, `name`, and `permissions` setters generated // [!code highlight]
    .build();

assert_eq!(user.name, "anon");
assert_eq!(user.level, 0);
assert_eq!(user.permissions, ["read"]);
```

:::

[automatic `Into` conversion qualification rules]: ../guide/into-conversions#types-that-qualify-for-an-automatic-into-conversion

*[Member]: Struct field or a function argument
*[member]: Struct field or a function argument
*[members]: Struct fields or function arguments
