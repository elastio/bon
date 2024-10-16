| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo build -p compilation-benchmarks --features=structs_100_fields_10,bon` | 2.316 ± 0.022 | 2.278 | 2.352 | 18.55 ± 0.98 |
| `cargo build -p compilation-benchmarks --features=structs_100_fields_10,typed-builder` | 1.882 ± 0.014 | 1.866 | 1.912 | 15.08 ± 0.79 |
| `cargo build -p compilation-benchmarks --features=structs_100_fields_10,derive_builder` | 1.055 ± 0.019 | 1.023 | 1.094 | 8.45 ± 0.47 |
| `cargo build -p compilation-benchmarks --features=structs_100_fields_10,` | 0.125 ± 0.007 | 0.107 | 0.135 | 1.00 |
