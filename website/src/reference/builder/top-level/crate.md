# `crate`

**Applies to:** <Badge text="structs"/> <Badge text="functions"/> <Badge text="methods"/><sup>(\*)</sup>

Overrides path to `bon` crate referenced in the generated code, which is useful in cases when `bon` macros are wrapped by other macros and `bon` crate is reexported.

::: info

(\*) `#[builder(crate)]` attribute isn't accepted on associated methods. Instead, you should use the `#[bon(crate)]` attribute on top of the impl block (see examples below).

:::

## Examples

::: code-group

```rust ignore [Struct]
#[derive(::path::to::bon::Builder)]
#[builder(crate = ::path::to::bon)]
struct Example {}
```

```rust ignore [Function]
#[::path::to::bon::builder(crate = ::path::to::bon)]
fn example() {}
```

```rust ignore [Method]
struct Example;

#[::path::to::bon::bon(crate = ::path::to::bon)]
impl Example {
    #[builder]
    fn example() {}
}
```

:::

## Compile Errors

The macro disallows relative paths. Only the following is accepted:

- Absolute path with a leading colon like `::foo::bar`
- Path with `crate` prefix like `crate::foo::bar`
- Path with `$crate` prefix like `$crate::foo::bar`
