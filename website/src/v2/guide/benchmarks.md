# Benchmarks

`#[builder]` generates code that is easily optimizable by the compiler. This has been tested by the benchmarks below. The benchmarks compare regular positional function call syntax and builder syntax for functions annotated with `#[builder]`.

In many cases `rustc` generates the same assembly code for the builder syntax as it would for a regular function call. Even when the generated assembly differs, the performance differences are negligible.

::: tip TIP

Don't take these microbenchmarks for granted. Do your own performance measurements in your application in real conditions. Feel free to [open an issue](https://github.com/elastio/bon/issues) if you find performance problems in `bon`.

:::

## Wallclock statistics

| Benchmark         | Description                                   | Assembly output                                      | Run time                                              |
| ----------------- | --------------------------------------------- | ---------------------------------------------------- | ----------------------------------------------------- |
| `args_3`          | 3 args of primitive types                     | [Equal](https://godbolt.org/z/YbTc4xGGY)             | regular:&nbsp;`6.6536ns`<br/>builder:&nbsp;`6.6494ns` |
| `args_5`          | 5 args of primitive types                     | [Equal](https://godbolt.org/z/TM3E7M6b3)             | regular:&nbsp;`7.9592ns`<br/>builder:&nbsp;`7.9731ns` |
| `args_10`         | 10 args of primitive types                    | [Ordering diff](https://godbolt.org/z/1d1fa38co)     | regular:&nbsp;`18.082ns`<br/>builder:&nbsp;`18.217ns` |
| `args_10_structs` | 10 args of primitive types and structs        | [Equal](https://godbolt.org/z/d6nn16E8q)             | regular:&nbsp;`9.2492ns`<br/>builder:&nbsp;`9.2325ns` |
| `args_10_alloc`   | 10 args of primitive and heap-allocated types | [Instructions diff](https://godbolt.org/z/fEMvnWvbc) | regular:&nbsp;`86.090ns`<br/>builder:&nbsp;`86.790ns` |
| `args_20`         | 20 args of primitive types                    | [Ordering diff](https://godbolt.org/z/3czM3h68s)     | regular:&nbsp;`36.121ns`<br/>builder:&nbsp;`36.298ns` |

## High-precision statistics

| Benchmark         | Instructions count                            | L1&nbsp;accesses                              | L2&nbsp;accesses                        | RAM&nbsp;accesses                         |
| ----------------- | --------------------------------------------- | --------------------------------------------- | --------------------------------------- | ----------------------------------------- |
| `args_3`          | regular:&nbsp;`108`<br/>builder:&nbsp;`108`   | regular:&nbsp;`138`<br/>builder:&nbsp;`138`   | regular:&nbsp;`2`<br/>builder:&nbsp;`2` | regular:&nbsp;`4`<br/>builder:&nbsp;`4`   |
| `args_5`          | regular:&nbsp;`126`<br/>builder:&nbsp;`126`   | regular:&nbsp;`161`<br/>builder:&nbsp;`161`   | regular:&nbsp;`2`<br/>builder:&nbsp;`2` | regular:&nbsp;`10`<br/>builder:&nbsp;`10` |
| `args_10`         | regular:&nbsp;`281`<br/>builder:&nbsp;`281`   | regular:&nbsp;`381`<br/>builder:&nbsp;`380`   | regular:&nbsp;`2`<br/>builder:&nbsp;`2` | regular:&nbsp;`19`<br/>builder:&nbsp;`20` |
| `args_10_structs` | regular:&nbsp;`75`<br/>builder:&nbsp;`75`     | regular:&nbsp;`106`<br/>builder:&nbsp;`106`   | regular:&nbsp;`4`<br/>builder:&nbsp;`4` | regular:&nbsp;`12`<br/>builder:&nbsp;`12` |
| `args_10_alloc`   | regular:&nbsp;`2028`<br/>builder:&nbsp;`2027` | regular:&nbsp;`2824`<br/>builder:&nbsp;`2824` | regular:&nbsp;`3`<br/>builder:&nbsp;`2` | regular:&nbsp;`36`<br/>builder:&nbsp;`36` |
| `args_20`         | regular:&nbsp;`556`<br/>builder:&nbsp;`556`   | regular:&nbsp;`767`<br/>builder:&nbsp;`767`   | regular:&nbsp;`4`<br/>builder:&nbsp;`4` | regular:&nbsp;`36`<br/>builder:&nbsp;`36` |

## Conditions

The code was compiled with `opt-level = 3` and `debug = 0`.

### Hardware

The benchmarks were run on a dedicated root server `AX51-NVMe` on [Hetzner](https://www.hetzner.com/).

-   OS: Ubuntu 22.04.4 (Linux 5.15.0-76-generic)
-   CPU: AMD Ryzen 7 3700X 8-Core Processor (x86_64)
-   RAM: 62.8 GiB

## References

The source code of the benchmarks is [available here](https://github.com/elastio/bon/tree/master/benchmarks).
