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
| `args_3`          | regular:&nbsp;`71`<br/>builder:&nbsp;`71`     | regular:&nbsp;`81`<br/>builder:&nbsp;`81`     | regular:&nbsp;`1`<br/>builder:&nbsp;`1` | regular:&nbsp;`10`<br/>builder:&nbsp;`9`  |
| `args_5`          | regular:&nbsp;`89`<br/>builder:&nbsp;`89`     | regular:&nbsp;`111`<br/>builder:&nbsp;`111`   | regular:&nbsp;`0`<br/>builder:&nbsp;`0` | regular:&nbsp;`10`<br/>builder:&nbsp;`10` |
| `args_10`         | regular:&nbsp;`206`<br/>builder:&nbsp;`206`   | regular:&nbsp;`269`<br/>builder:&nbsp;`268`   | regular:&nbsp;`0`<br/>builder:&nbsp;`0` | regular:&nbsp;`20`<br/>builder:&nbsp;`21` |
| `args_10_structs` | regular:&nbsp;`20`<br/>builder:&nbsp;`20`     | regular:&nbsp;`29`<br/>builder:&nbsp;`28`     | regular:&nbsp;`0`<br/>builder:&nbsp;`0` | regular:&nbsp;`5`<br/>builder:&nbsp;`6`   |
| `args_10_alloc`   | regular:&nbsp;`1830`<br/>builder:&nbsp;`1829` | regular:&nbsp;`2555`<br/>builder:&nbsp;`2554` | regular:&nbsp;`1`<br/>builder:&nbsp;`1` | regular:&nbsp;`36`<br/>builder:&nbsp;`36` |
| `args_20`         | regular:&nbsp;`414`<br/>builder:&nbsp;`414`   | regular:&nbsp;`548`<br/>builder:&nbsp;`547`   | regular:&nbsp;`0`<br/>builder:&nbsp;`0` | regular:&nbsp;`46`<br/>builder:&nbsp;`47` |

## Conditions

The code was compiled with `opt-level = 3` and `debug = 0`.

### Hardware

The benchmarks were run on a dedicated root server `AX51-NVMe` on [Hetzner](https://www.hetzner.com/).

- OS: Ubuntu 22.04.4 (Linux 5.15.0-76-generic)
- CPU: AMD Ryzen 7 3700X 8-Core Processor (x86_64)
- RAM: 62.8 GiB

## References

The source code of the benchmarks is [available here](https://github.com/elastio/bon/tree/master/benchmarks).
