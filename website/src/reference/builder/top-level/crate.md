# `crate`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/><sub>(*)</sub>

Overrides path to `bon` crate referenced in the generated code, which is useful in cases when `bon` macros are wrapped by other macros and `bon` crate is reexported.

::: info

(*) `#[builder(crate)]` attribute isn't directly supported for associated methods. Instead, you should use the `#[bon(crate)]` attribute on top of the impl block (see examples below).

:::

## Examples

::: code-group

```rust ignore [Struct]
#[derive(::path::to::bon::Builder)]
#[builder(crate = ::path::to::bon)]
struct Example {}
```

```rust ignore [Free function]
#[::path::to::bon::builder(crate = ::path::to::bon)]
fn example() {}
```

```rust ignore [Associated method]
struct Example;

#[::path::to::bon::bon(crate = ::path::to::bon)]
impl Example {
    #[builder]
    fn example() {}
}
```

:::


## Compile errors

The macro disallows relative paths. Only the following is accepted:
- Absolute path with a leading colon like `::foo::bar`
- Path with `crate` prefix like `crate::foo::bar`
- Path with `$crate` prefix like `$crate::foo::bar`
