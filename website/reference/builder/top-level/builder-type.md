# `builder_type`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/>

Overrides the name of the generated builder struct.

The default naming pattern is the following:

| Underlying item            | Naming pattern                                |
| -------------------------- | --------------------------------------------- |
| Struct                     | `{StructName}Builder`                         |
| `StructName::new()` method | `{StructName}Builder`                         |
| Free function              | `{PascalCaseFunctionName}Builder`             |
| Associated method          | `{SelfTypeName}{PascalCaseMethodName}Builder` |

The attribute expects the desired builder type identifier as its input.

**Example:**

::: code-group

```rust [Struct]
use bon::Builder;

#[derive(Builder)]
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
use bon::{builder, Builder};

#[derive(Builder)] // [!code error]
struct Brush {}

#[builder] // [!code error]
fn brush() {}

// `BrushBuilder` builder type name was generated for both
// the struct and the function. This is a compile error
let builder: BrushBuilder = Brush::builder();
let builder: BrushBuilder = brush();
```

```rust [Fixed]
use bon::{builder, Builder};

#[derive(Builder)]
#[builder(builder_type = MyBuilder)] // [!code highlight]
struct Brush {}

#[builder]
fn brush() {}

// Now builder types are named differently
let builder: MyBuilder = Brush::builder();
let builder: BrushBuilder = brush();
```

:::
