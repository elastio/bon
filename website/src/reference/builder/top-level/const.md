# `const`

**Applies to:** <Badge text="structs"/> <Badge text="functions"/> <Badge text="methods"/>

Marks all generated builder functions and methods as `const fn`. See the [limitations](#limitations) of this feature.

## Examples

::: code-group

```rust ignore [Struct]
use bon::Builder;

#[derive(Builder)]
#[builder(const)]
struct Example {
    x1: u32,

    // Must specify the default value explicitly because `Default::default()`
    // is not callable in `const` context
    #[builder(default = 0)]
    x2: u32
}

const {
    Example::builder()
        .x1(32)
        .build();
}
```

```rust ignore [Function]
use bon::builder;

#[builder(const)]
fn example(
    x1: u32,

    // Must specify the default value explicitly because `Default::default()`
    // is not callable in `const` context
    #[builder(default = 0)]
    x2: u32
) {}

const {
    example()
        .x1(32)
        .call();
}
```

```rust ignore [Method]
use bon::bon;

struct Example;

#[bon]
impl Example {
    #[builder(const)]
    fn example(
        x1: u32,

        // Must specify the default value explicitly because `Default::default()`
        // is not callable in `const` context
        #[builder(default = 0)]
        x2: u32
    ) {}
}

const {
    Example::example()
        .x1(32)
        .call();
}
```

:::

## Limitations

| Limitation                                                                             | Reason                                                                                                                                                        |
| -------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `const` must be the _first_ attribute parameter: `#[builder(const, ...other params)]`  | `syn::Meta` [limitation](https://github.com/dtolnay/syn/issues/1458)                                                                                          |
| No member types with non-trivial `Drop` implementations such as `String`, `Vec`, `Box` | Requires&nbsp;[const_precise_live_drops](https://github.com/rust-lang/rust/issues/73255)                                                                      |
| No _bare_ [`default`] and [`skip`]                                                     | `Default::default()` can't be called in `const` context. Specify the value explicitly via `default/skip = ...`                                                |
| Only simple expressions are allowed in [`default`], [`skip`], [`with`]                 | Requires [const_closures](https://github.com/rust-lang/rust/issues/106003). In the meantime move your expression into a separate function if `bon` rejects it |
| `1.61.0` MSRV                                                                          | Requires [`const_fn_trait_bound`](https://github.com/rust-lang/rust/issues/93706)                                                                             |

[`default`]: ../member/default
[`skip`]: ../member/skip
[`with`]: ../member/with
