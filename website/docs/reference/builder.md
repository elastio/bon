# `#[builder]`

## Attributes applicability

The attributes described below are all expected to be specified inside of the parentheses in the macro invocation like this:

```js
#[builder(parameter_name = some_value_here)]
```

Pay attention to the **"Applies to"** clause of each attribute that describes the contexts where the parameter is allowed. Examples of each context are shown below.

:::details <Badge text="free functions"/> <Badge type="warning" text="free function arguments"/>
```rust
use bon::builder;

// free functions                       // [!code highlight]
#[builder(finish_fn = name1)]           // [!code highlight]
fn foo(
    // free function arguments          // [!code highlight]
    #[builder(into)]                    // [!code highlight]
    arg: u32
) {}
```
:::

:::details <Badge text="structs"/> <Badge type="warning" text="struct fields"/> <Badge text="associated methods"/> <Badge type="warning" text="associated method arguments"/>
```rust
use bon::{bon, builder};

// structs                              // [!code highlight]
#[builder(finish_fn = name2)]           // [!code highlight]
struct Foo {
    // struct fields                    // [!code highlight]
    #[builder(into)]                    // [!code highlight]
    bar: u32,
}

#[bon]
impl Foo {
    // associated methods               // [!code highlight]
    #[builder(finish_fn = name3)]       // [!code highlight]
    fn foo(
        // associated method arguments  // [!code highlight]
        #[builder(into)]                // [!code highlight]
        arg: u32
    ) {}
}
```
:::

## `builder_type`

**Applies to:** <Badge text="free functions"/> <Badge text="associated methods"/> <Badge text="structs"/>

Overrides the name of the generated builder struct.

The default naming pattern is the following.

Type of item `#[builder]` is placed on | Naming pattern
---------------------------------------|----------------------
Struct                                 | `{StructName}Builder`
Free function                          | `{PascalCaseFunctionName}Builder`
Associated method                      | `{SelfTypeName}{PascalCaseMethodName}Builder`

**Example:**

```rust
use bon::builder;

#[builder(builder_type = MyBuilder)]
struct Counter {}

let builder: MyBuilder = Counter::builder();


```
TODO: finish this section


## `finish_fn`

**Applies to:** <Badge text="free functions"/> <Badge text="associated methods"/> <Badge text="structs"/>

This attribute allows overriding the name of the generated builder's method that finishes the building process.

**Example:**

```rust
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

## `default`

**Applies to:** <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/> <Badge type="warning" text="struct fields"/>

Makes the struct field or function argument optional. If the caller doesn't specify a value for this parameter via the setter methods, then the default value will be computed based on the form of this attribute:

- `#[builder(default)]` - default value will be computed using the `Default` trait
- `#[builder(default = expression)]` - default value will be computed using the provided `expression`.

The result of the `expression` will automatically be converted into the target type if `Into` conversion is enabled for this setter i.e. the type satisfies [automatic `Into` conversion qualification rules], or there is a [`#[builder(into)]`](#into) override.

**Example:**

```rust
use bon::builder;

#[builder]
struct User {
    #[builder(default)] // [!code highlight]
    level: u32,

    // The default value expression of type `&'static str` is // [!code highlight]
    // automatically converted to `String` here via `Into`.   // [!code highlight]
    #[builder(default = "anon")]                              // [!code highlight]
    name: String,

    // Any complex expression is accepted   // [!code highlight]
    #[builder(default = bon::vec!["read"])] // [!code highlight]
    permissions: Vec<String>,
}

let user = User::builder().build();

assert_eq!(user.name, "anon");

// `<u32 as Default>::default()` is zero
assert_eq!(user.level, 0);

assert_eq!(user.permissions, ["read"]);
```

### Compile errors

This attribute is incompatible with struct fields or function arguments of `Option` type, since `Option` already implies the default value of `None`.

### API compatibility

Specifying this attribute is equivalent to wrapping its type with `Option<...>`. The same setter methods as for `Option<...>` fields will be generated. This means you can freely switch between using `#[builder(default)]` and manually wrapping the struct field or function argument type with an `Option<...>`.

## `into`

**Applies to:** <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/> <Badge type="warning" text="struct fields"/>

This attribute forces an `impl Into` conversion to be enabled or disabled in the generated setter method. Use this to force-override the decision made by [automatic `Into` conversion qualification rules].

This parameter can be specified in one of the following ways:

Form                      | Behavior
--------------------------|----------------------------------------------
`#[builder(into)]`        | Forcefully **enables** `impl Into` in the setter method
`#[builder(into = false)]`| Forcefully **disables** `impl Into` in the setter method

**Example:**

```rust
use bon::builder;
use std::num::NonZeroU32;

#[builder]
struct Counter {
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

Counter::builder()
    // setter accepts `impl Into<u32>`                        // [!code highlight]
    .force_enabled_into(non_zero_u32)                         // [!code highlight]
    // setter accepts `String` instead of `impl Into<String>` // [!code highlight]
    .force_disabled_into("".to_owned())                       // [!code highlight]
    .build();
```

### Compile errors

If the placement of this attribute wouldn't override the default behavior, a compile error will be generated requesting the removal of a redundant attribute.

For example, in the following code `String` already qualifies for an `Into` conversion according to the [automatic `Into` conversion qualification rules].

```rust compile_fail
use bon::builder;

#[builder]
fn redundant_attribute_error(
    // Compile error: "This attribute is redundant and can be removed." // [!code error]
    #[builder(into)] // [!code error]
    string: String
) {}
```

[automatic `Into` conversion qualification rules]: ../guide/into-conversions#types-that-qualify-for-an-automatic-into-conversion
