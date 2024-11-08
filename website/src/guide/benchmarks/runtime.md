# Runtime Benchmarks

Builder macros generate code that is easily optimizable by the compiler. This has been tested by the benchmarks below. The benchmarks compare regular positional function call syntax and builder syntax for functions annotated with `#[builder]`.

In many cases `rustc` generates the same assembly code for the builder syntax as it would for a regular function call. Even when the generated assembly differs, the performance differences are negligible.

::: tip TIP

Don't take these microbenchmarks for granted. Do your own performance measurements in your application in real conditions. Feel free to [open an issue](https://github.com/elastio/bon/issues) if you find performance problems in `bon`.

:::

<!-- Prevent separating wrapping in tables -->
<style>
.bon-wallclock-stats-table tr > td:not(:nth-child(2))  {
    white-space: nowrap;
}
.bon-high-precision-stats-table tr > td {
    white-space: nowrap;
}
</style>

## Wallclock Statistics

<div class="bon-wallclock-stats-table">

| Benchmark         | Description                                   | Assembly output                                      | Run time                                         |
| ----------------- | --------------------------------------------- | ---------------------------------------------------- | ------------------------------------------------ |
| `args_3`          | 3 args of primitive types                     | [Equal](https://godbolt.org/z/xKvqr35TM)             | regular: `6.2751ns`<br/>builder: `6.3021ns`      |
| `args_5`          | 5 args of primitive types                     | [Equal](https://godbolt.org/z/oPc35ees5)             | regular: `7.8298ns`<br/>builder: `7.8321ns`      |
| `args_10`         | 10 args of primitive types                    | [Ordering diff](https://godbolt.org/z/Ys9EszPTv)     | regular: `17.322ns`<br/>builder: `17.178ns`      |
| `args_10_structs` | 10 args of primitive types and structs        | [Instructions diff](https://godbolt.org/z/YxjdGMncs) | regular: `2.7477ns`<br/>builder: `2.7311ns`      |
| `args_10_alloc`   | 10 args of primitive and heap-allocated types | [Instructions diff](https://godbolt.org/z/chdnTYdqh) | regular: `91.666ns`<br/>builder: `84.818ns` (\*) |
| `args_20`         | 20 args of primitive types                    | [Equal](https://godbolt.org/z/13ncxPT5s)             | regular: `36.467ns`<br/>builder: `36.786ns`      |

</div>

::: tip (\*)

Interestingly, in this case builder version performed even better. If you don't believe this, you can run these benchmarks for [yourself][benchmarks-source]. Maybe some ASM expert could explain this 😳?

:::

## High-Precision Statistics

<div class="bon-high-precision-stats-table">

| Benchmark         | Instructions count                  | L1 accesses                         | L2 accesses                   | RAM accesses                    |
| ----------------- | ----------------------------------- | ----------------------------------- | ----------------------------- | ------------------------------- |
| `args_3`          | regular: `107`<br/>builder: `107`   | regular: `134`<br/>builder: `134`   | regular: `1`<br/>builder: `1` | regular: `8`<br/>builder: `8`   |
| `args_5`          | regular: `125`<br/>builder: `125`   | regular: `164`<br/>builder: `164`   | regular: `1`<br/>builder: `1` | regular: `7`<br/>builder: `7`   |
| `args_10`         | regular: `283`<br/>builder: `283`   | regular: `382`<br/>builder: `383`   | regular: `4`<br/>builder: `2` | regular: `18`<br/>builder: `19` |
| `args_10_structs` | regular: `22`<br/>builder: `22`     | regular: `30`<br/>builder: `31`     | regular: `2`<br/>builder: `1` | regular: `5`<br/>builder: `5`   |
| `args_10_alloc`   | regular: `2038`<br/>builder: `2037` | regular: `2839`<br/>builder: `2837` | regular: `1`<br/>builder: `1` | regular: `33`<br/>builder: `34` |
| `args_20`         | regular: `557`<br/>builder: `557`   | regular: `775`<br/>builder: `775`   | regular: `1`<br/>builder: `1` | regular: `32`<br/>builder: `32` |

</div>

## Conditions

The code was compiled with `opt-level = 3` and `debug = 0`.

### Hardware

The benchmarks were run on a dedicated root server `AX51-NVMe` on [Hetzner](https://www.hetzner.com/).

-   OS: Ubuntu 22.04.4 (Linux 5.15.0-76-generic)
-   CPU: AMD Ryzen 7 3700X 8-Core Processor (x86_64)
-   RAM: 62.8 GiB

## References

The source code of the benchmarks is [available here][benchmarks-source].

[benchmarks-source]: https://github.com/elastio/bon/tree/master/benchmarks/runtime
