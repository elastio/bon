# Benchmarks

`#[builder]` generates code that is easily optimizable by the compiler. This has been tested by the benchmarks below. The benchmarks compare regular positional function call syntax and builder syntax for functions annotated with `#[builder]`.

In many cases `rustc` generates the same assembly code for the builder syntax as it would for a regular function call. Even when there are differences in the generated assembly, they don't influence the performance (perf. is within measurement error margin).

## Summary

| Benchmark         | Description                                   | Assembly output                                      | Run time
| --                | --                                            | --                                                   | --
| `args_3`          | 3 args of primitive types                     | [Equal](https://godbolt.org/z/cc4ao8x6W)             | regular:&nbsp;`6.6536ns`<br/>builder:&nbsp;`6.6494ns`
| `args_5`          | 5 args of primitive types                     | [Equal](https://godbolt.org/z/M93M3Yfsj)             | regular:&nbsp;`7.9592ns`<br/>builder:&nbsp;`7.9731ns`
| `args_10`         | 10 args of primitive types                    | [Ordering diff](https://godbolt.org/z/1c9P5Gjrv)     | regular:&nbsp;`18.082ns`<br/>builder:&nbsp;`18.217ns`
| `args_10_structs` | 10 args of primitive types and structs        | [Equal](https://godbolt.org/z/95vcn78Tn)             | regular:&nbsp;`5.0784ns`<br/>builder:&nbsp;`5.0481ns`
| `args_10_alloc`   | 10 args of primitive and heap-allocated types | [Instructions diff](https://godbolt.org/z/bzEbqrvPW) | regular:&nbsp;`86.090ns`<br/>builder:&nbsp;`86.790ns`
| `args_20`         | 20 args of primitive types                    | [Ordering diff](https://godbolt.org/z/GqP44GxnW)     | regular:&nbsp;`37.381ns`<br/>builder:&nbsp;`37.623ns`

These statistics were collected by [`criterion`](https://github.com/bheisler/criterion.rs)

::: details Detailed statistics

| Benchmark         | Instructions count                           | L1&nbsp;accesses                                   | L2&nbsp;accesses                             | RAM&nbsp;accesses
| --                | --                                           | --                                            | --                                      | --
| `args_3`          | regular:&nbsp;`106`<br/>builder:&nbsp;`106`  | regular:&nbsp;`136`<br/>builder:&nbsp;`136`   | regular:&nbsp;`2`<br/>builder:&nbsp;`2` | regular:&nbsp;`4`<br/>builder:&nbsp;`4`
| `args_5`          | regular:&nbsp;`124`<br/>builder:&nbsp;`124`  | regular:&nbsp;`161`<br/>builder:&nbsp;`161`   | regular:&nbsp;`1`<br/>builder:&nbsp;`1` | regular:&nbsp;`9`<br/>builder:&nbsp;`9`
| `args_10`         | regular:&nbsp;`281`<br/>builder:&nbsp;`281`  | regular:&nbsp;`381`<br/>builder:&nbsp;`380`   | regular:&nbsp;`2`<br/>builder:&nbsp;`2` | regular:&nbsp;`19`<br/>builder:&nbsp;`20`
| `args_10_structs` | regular:&nbsp;`73`<br/>builder:&nbsp;`73`    | regular:&nbsp;`108`<br/>builder:&nbsp;`108`   | regular:&nbsp;`2`<br/>builder:&nbsp;`2` | regular:&nbsp;`10`<br/>builder:&nbsp;`10`
| `args_10_alloc`   | regular:&nbsp;`2025`<br/>builder:&nbsp;`2026`| regular:&nbsp;`2823`<br/>builder:&nbsp;`2823` | regular:&nbsp;`3`<br/>builder:&nbsp;`2` | regular:&nbsp;`35`<br/>builder:&nbsp;`35`
| `args_20`         | regular:&nbsp;`554`<br/>builder:&nbsp;`554`  | regular:&nbsp;`768`<br/>builder:&nbsp;`769`   | regular:&nbsp;`3`<br/>builder:&nbsp;`3` | regular:&nbsp;`34`<br/>builder:&nbsp;`33`

These statistics were collected by [`iai`](https://github.com/bheisler/iai)

:::

---

The source code of the benchmarks is [available here](https://github.com/elastio/bon/tree/master/benchmarks).
