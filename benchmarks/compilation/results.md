| Command                                | Mean Wall Time [s] | Change | Factor         |
| :------------------------------------- | -----------------: | -----: | :------------- |
| `structs_100_fields_10 bon`            |      1.612 ± 0.014 |        |                |
| `structs_100_fields_10 typed-builder`  |      1.723 ± 0.034 |  +6.9% |                |
| `structs_100_fields_10 derive_builder` |      1.055 ± 0.006 | -34.6% | (1.5x faster)  |
| `structs_100_fields_10 `               |      0.114 ± 0.004 | -92.9% | (14.1x faster) |
| `structs_10_fields_50 bon`             |      1.557 ± 0.030 |  -3.4% |                |
| `structs_10_fields_50 typed-builder`   |      2.059 ± 0.030 | +27.7% |                |
| `structs_10_fields_50 derive_builder`  |      0.407 ± 0.002 | -74.7% | (4.0x faster)  |
| `structs_10_fields_50 `                |      0.094 ± 0.001 | -94.2% | (17.2x faster) |
