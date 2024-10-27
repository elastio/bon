
# `finish_fn`

**Applies to:** <Badge type="warning" text="struct fields"/> <Badge type="warning" text="free function arguments"/> <Badge type="warning" text="associated method arguments"/>

Makes the member a positional argument on the finishing function that consumes the builder and returns the resulting object (for struct syntax) or performs the requested action (for function/method syntax).

The ordering of members annotated with `#[builder(finish_fn)]` matters! They will appear in the same order relative to each other in the finishing function signature. They must also be declared at the top of the members list strictly after members annotated with [`#[builder(start_fn)]`](./start-fn) (if any).

This ensures a consistent initialization order, and it makes these members available for expressions in `#[builder(default/skip = ...)]` for regular members that follow them.

::: tip

Don't confuse this with the top-level [`#[builder(finish_fn = ...)]`](../top-level/finish-fn) attribute that can be used to configure the name and visibility of the finishing function. You'll likely want to use it in combination with this member-level attribute to define a better name for the finishing function.

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
