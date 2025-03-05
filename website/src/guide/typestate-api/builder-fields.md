# Builder Fields

On this page, you'll learn how to access some [native fields](#native-fields) of the builder type and add [custom fields](#custom-fields) to it 🌾.

## Native Fields

There are some native module-private fields on the builder that _you can access_ as the user of `bon`'s builder macros. The fields described below are guaranteed to have stable names and types so you can use them in custom methods such as custom setters and getters.

::: warning Important Privacy Clarification

The native fields documented here have default private Rust visibility. They are declared like this in the builder struct:

```rust compile_fail
struct BuilderStruct {
    self_receiver: T,
    // ... other not documented fields are actually private impl. details,
    // so don't even try to access fields that begin with `__unsafe_private`!
}
```

This means the fields documented below are accessible within the module where the builder struct was generated, but if you try to access them outside this module's scope, you'll get a compile error, because the fields aren't marked as `pub` or `pub(...)`.

:::

### `self_receiver`

The field `self_receiver` is available on a builder generated from a method that has a `self` parameter.

::: tip

The term "receiver" was borrowed from [`syn`'s](https://docs.rs/syn/latest/syn/struct.Receiver.html) and [Rust Reference](https://doc.rust-lang.org/reference/expressions/method-call-expr.html) terminology.

:::

```rust
use bon::bon;

struct Example {
    x1: u32
}

#[bon]
impl Example {
    #[builder]
    fn method(&self) {
        // ...
    }
}

let builder = Example { x1: 99 }.method();

// We can access the `self_receiver` on the builder. // [!code highlight]
// It is a reference in this case, because `method` takes a `&self` // [!code highlight]
let self_receiver: &Example = builder.self_receiver;

assert_eq!(self_receiver.x1, 99);
```

### `#[builder(start_fn)]` members

You can access the values of members marked with [`#[builder(start_fn)]`](../../reference/builder/member/start_fn) by their name on the builder directly.

```rust
use bon::Builder;

#[derive(Builder)]
struct Example {
    #[builder(start_fn)]
    x1: u32
}

let builder = Example::builder(99);

// We can access the `x1` on the builder directly. // [!code highlight]
let x1: u32 = builder.x1;

assert_eq!(x1, 99);
```

## Custom Fields

This is useful if you'd like a completely custom state for [custom setters](./custom-methods) in the builder.

To understand how it works, we'll create a builder API _similar_ to [`std::process::Command`](https://doc.rust-lang.org/stable/std/process/struct.Command.html) where you have a couple of methods [`arg`](https://doc.rust-lang.org/stable/std/process/struct.Command.html#method.arg) and [`args`](https://doc.rust-lang.org/stable/std/process/struct.Command.html#method.args), that push values into an internal arguments `Vec`.

We'll use the [`#[builder(field)]`](../../reference/builder/member/field) attribute to define a custom field, that will be accessible in our `arg/args` custom setters. The value of this field will be moved into the resulting struct or function from which the builder was generated.

::: tip

See [Custom Methods](./custom-methods) for details on how to write an impl block with additional methods for the builder if you haven't already.

:::

```rust
use bon::Builder;

#[derive(Builder)]
struct Command {
    // Define a private field on the builder without setters. // [!code highlight]
    // It's initialized with `Default::default()` at the start // [!code highlight]
    #[builder(field)] // [!code highlight]
    args: Vec<String>,

    #[builder(into)]
    name: String,
}

// This is the API we'd like to have // [!code highlight]
let cmd = Command::builder()
    .name("ls")
    .arg("-l")
    .args(["foo", "bar"])
    .build();

assert_eq!(cmd.name, "ls");
assert_eq!(cmd.args, ["-l", "foo", "bar"]);

// Now define custom `arg/args` methods on the builder itself. // [!code highlight]
impl<S: command_builder::State> CommandBuilder<S> {
    fn arg(mut self, arg: impl Into<String>) -> Self {
        // We have access to `self.args` private 🔒 field on `CommandBuilder`! // [!code highlight]
        self.args.push(arg.into()); // [!code highlight]
        self
    }

    fn args(mut self, args: impl IntoIterator<Item: Into<String>>) -> Self {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }
}
```

And that's it! This way you can extend `bon`'s builders with almost any state and behaviour, that you want.

You can specify a custom initial value with `#[builder(field = expr)]`. That `expr` can refer to other members and fields defined higher. See the [evaluation context reference](../../reference/builder/member/field#evaluation-context) for details.
