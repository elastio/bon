| Command | Mean [s] | Min [s] | Max [s] | Relative |
|:---|---:|---:|---:|---:|
| `cargo build -p compilation-benchmarks --features=bon` | 2.522 ± 0.069 | 2.467 | 2.710 | 8.13 ± 0.58 |
| `cargo build -p compilation-benchmarks --features=bon-overwritable` | 2.413 ± 0.021 | 2.383 | 2.439 | 7.78 ± 0.52 |
| `cargo build -p compilation-benchmarks --features=typed-builder` | 2.076 ± 0.022 | 2.042 | 2.107 | 6.69 ± 0.45 |
| `cargo build -p compilation-benchmarks --features=derive_builder` | 1.243 ± 0.021 | 1.194 | 1.262 | 4.01 ± 0.27 |
| `cargo build -p compilation-benchmarks --features=` | 0.310 ± 0.020 | 0.258 | 0.332 | 1.00 |
