# concolor-control

> **bin/lib API for managing terminal styling**

[![Documentation](https://img.shields.io/badge/docs-master-blue.svg)][Documentation]
![License](https://img.shields.io/crates/l/conclor-query.svg)
[![Crates Status](https://img.shields.io/crates/v/concolor-query.svg)](https://crates.io/crates/concolor-query)

Features
- Detects interactive `stdout` / `stderr`
- Detects terminal capabilities via `TERM`
- Detects and enables ANSI support on Windows
- Supports [CLICOLOR] and [NO_COLOR]

## [Contribute](../../CONTRIBUTING.md)

## Special Thanks

Prior art for global colors control:

- [yansi](https://crates.io/crates/yansi)
- [clicolors-control](https://crates.io/crates/clicolors-control)

[termcolor](https://crates.io/crates/termcolor) for identifying various corner cases with environment detection.

[firestorm](https://crates.io/crates/firestorm) for zero-cost abstraction via bin/lib-specific `Cargo.toml` features.

## License

Dual-licensed under [MIT](../../LICENSE-MIT) or [Apache 2.0](../../LICENSE-APACHE)

[Documentation]: https://docs.rs/concolor-query
[CLICOLOR]: https://bixense.com/clicolors/
[NO_COLOR]: https://no-color.org/
