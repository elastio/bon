# `crate`

**Applies to:** <Badge text="structs"/> <Badge text="free functions"/> <Badge text="associated methods"/><sub>(*)</sub>

Overrides the path to `bon` crate referenced in the generated code, which is useful in cases when `bon` macros are wrapped by other macros.

(*) `#[builder(crate)]` attribute isn't supported for associated methods. Instead, you should use the `#[bon(crate)]` attribute on top of the impl block.


## Examples

Suppose this code lives in the crate `my_lib`:

::: code-group

```rust [Struct]
// Reexport `bon`, so it can be referenced from the macro-generated code
#[doc(hidden)]
pub use bon;

#[macro_export]
macro_rules! gen_builder {
    ($struct_item:item) => {
        #[derive($crate::bon::Builder)]
        #[builder(crate = $crate::bon)]
        $struct_item
    }
}
```

```rust [Free function]
// Reexport `bon`, so it can be referenced from the macro-generated code
#[doc(hidden)]
pub use bon;

#[macro_export]
macro_rules! gen_builder {
    ($fn_item:item) => {
        #[$crate::bon::builder(crate = $crate::bon)]
        $fn_item
    }
}
```

```rust [Associated method]
// Reexport `bon`, so it can be referenced from the macro-generated code
#[doc(hidden)]
pub use bon;

#[macro_export]
macro_rules! gen_builder {
    ($impl_item:item) => {
        #[$crate::bon::bon(crate = $crate::bon)]
        $impl_item
    }
}
```

:::

When you use this macro wrapper in the other crate, it will work fine:

::: code-group

```rust ignore [Struct]
my_lib::gen_builder! {
    struct Example {}
}
```

```rust ignore [Free function]
my_lib::gen_builder! {
    fn example() {}
}
```

```rust ignore [Associated method]
struct Example;

my_lib::gen_builder! {
    impl Example {
        #[builder]
        fn example() {}
    }
}
```

:::

Without the `crate` attribute this code wouldn't compile because the generated code would try to access symbols from `::bon` instead of from the `my_crate::bon` reexport.


## Compile errors

The macro disallows relative paths. Only th following is accepted:
- Absolute path with a leading colon like `::foo::bar`
- Path with `crate` prefix like `crate::foo::bar`
- Path with `$crate` prefix like `$crate::foo::bar`
