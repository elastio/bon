| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo build -p compilation-benchmarks --features=structs_100_fields_10,bon` | 2.292 ± 0.025 | 2.257 | 2.329 | 22.43 ± 3.37 |
| `cargo build -p compilation-benchmarks --features=structs_100_fields_10,bon-overwritable` | 2.216 ± 0.019 | 2.186 | 2.242 | 21.69 ± 3.26 |
| `cargo build -p compilation-benchmarks --features=structs_100_fields_10,typed-builder` | 1.884 ± 0.022 | 1.850 | 1.911 | 18.43 ± 2.77 |
| `cargo build -p compilation-benchmarks --features=structs_100_fields_10,derive_builder` | 1.063 ± 0.016 | 1.037 | 1.087 | 10.40 ± 1.57 |
| `cargo build -p compilation-benchmarks --features=structs_100_fields_10,` | 0.123 ± 0.015 | 0.097 | 0.146 | 1.21 ± 0.23 |
| `cargo build -p compilation-benchmarks --features=structs_10_fields_50,bon` | 2.051 ± 0.019 | 2.023 | 2.077 | 20.07 ± 3.02 |
| `cargo build -p compilation-benchmarks --features=structs_10_fields_50,bon-overwritable` | 1.999 ± 0.017 | 1.974 | 2.026 | 19.56 ± 2.94 |
| `cargo build -p compilation-benchmarks --features=structs_10_fields_50,typed-builder` | 2.098 ± 0.021 | 2.055 | 2.124 | 20.53 ± 3.09 |
| `cargo build -p compilation-benchmarks --features=structs_10_fields_50,derive_builder` | 0.453 ± 0.006 | 0.440 | 0.461 | 4.43 ± 0.67 |
| `cargo build -p compilation-benchmarks --features=structs_10_fields_50,` | 0.102 ± 0.015 | 0.088 | 0.133 | 1.00 |
