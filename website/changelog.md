# Changelog
All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.1.0](https://github.com/elastio/bon/compare/v2.0.1...v2.1.0) - 2024-09-01

See the [blog post for this release](https://elastio.github.io/bon/blog/bon-builder-generator-v2-release) that describes some of the most notable changes in detail.

### Added
- `#[must_use]` on the `build()` method for structs and `call()` for functions (if the original function has `#[must_use]`) ([#82](https://github.com/elastio/bon/pull/82)). Thanks [@EdJoPaTo](https://github.com/EdJoPaTo) for the contribution!

### Changed

- Optimize `bon`'s generated code type-checking performance and improve error messages ([#84](https://github.com/elastio/bon/pull/84))
- Improve builder() method docs ([#76](https://github.com/elastio/bon/pull/76)). Thanks [@EdJoPaTo](https://github.com/EdJoPaTo) for the contribution!

### Fixed
- Don't warn on `clippy::impl_trait_in_params` ([#80](https://github.com/elastio/bon/pull/80)). Thanks [@EdJoPaTo](https://github.com/EdJoPaTo) for the contribution!
- Fix typos in messages and code comments ([#79](https://github.com/elastio/bon/pull/79)). Thanks [@EdJoPaTo](https://github.com/EdJoPaTo) for the contribution!

### Other
- Add more tests for `#[must_use]` ([#87](https://github.com/elastio/bon/pull/87))


## [2.0.1](https://github.com/elastio/bon/compare/v2.0.0...v2.0.1) - 2024-08-28

### Docs
- Add a new section ["`None` literals inference"](https://elastio.github.io/bon/guide/patterns/into-conversions-in-depth#none-literals-inference) to docs for "Into Conversions In-Depth"
- Fix the docs about the comparison of Into conversions on the ["Alternatives"](http://elastio.github.io/bon/guide/alternatives) page that were not updated during the v2 release

### Fix
- Fix capturing of generic params that appear only in return types ([#72](https://github.com/elastio/bon/pull/72))
- Fix support for associated types ([#72](https://github.com/elastio/bon/pull/72))

### Internal
- Add more tests for various edge cases ([#70](https://github.com/elastio/bon/pull/70))

## [2.0.0](https://github.com/elastio/bon/compare/v1.2.1...v2.0.0) - 2024-08-26

See the [blog post](https://elastio.github.io/bon/blog/bon-builder-generator-v2-release) for details.

## [1.2.1](https://github.com/elastio/bon/compare/v1.2.0...v1.2.1) - 2024-08-12

### Other
- Remove unnecessary const block ([#52](https://github.com/elastio/bon/pull/52))
- Small cleanup ([#51](https://github.com/elastio/bon/pull/51))

## [1.2.0](https://github.com/elastio/bon/compare/v1.1.0...v1.2.0) - 2024-08-09

### Added
- Add `#[builder(skip)]` attribute to skip generating setters ([#44](https://github.com/elastio/bon/pull/44))
- Add automatic docs for setters ([#45](https://github.com/elastio/bon/pull/45))

### Other
- Remove dependencies on `easy-ext`, `heck` and `itertools` ([#42](https://github.com/elastio/bon/pull/42))

## [1.1.0](https://github.com/elastio/bon/compare/v1.0.6...v1.1.0) - 2024-08-07

### Added

- Add `no_std` support ([#36](https://github.com/elastio/bon/pull/36)). Thanks [@danielschemmel](https://github.com/danielschemmel) for the contribution!
- Add asm comparison and benchmarks to docs ([#29](https://github.com/elastio/bon/pull/29))
- Add `map!{}` and `set![]` macros ([#33](https://github.com/elastio/bon/pull/33)). Thanks [@korrat](https://github.com/korrat) for the contribution!

### Fixed

- Fix `missing_docs` lint in the generated code ([#39](https://github.com/elastio/bon/pull/39))

## [1.0.6](https://github.com/elastio/bon/compare/v1.0.5...v1.0.6) - 2024-08-01

### Fixed
- Explicitly specify the minimum required version of the darling dependency ([#30](https://github.com/elastio/bon/pull/30))

## [1.0.5](https://github.com/elastio/bon/compare/v1.0.4...v1.0.5) - 2024-07-31

### Added
- Add `#[must_use]` to the builder and other small improvements ([#26](https://github.com/elastio/bon/pull/26))

## [1.0.4](https://github.com/elastio/bon/compare/v1.0.3...v1.0.4) - 2024-07-30

### Fixed
- new() method is now hidden by default and the Builder type name is the same as when `#[builder]` is on top of a `struct` ([#19](https://github.com/elastio/bon/pull/19))

## [1.0.3](https://github.com/elastio/bon/compare/v1.0.2...v1.0.3) - 2024-07-30

### Fixed
- Fix missing captured generics on an impl block that aren't referenced in the method ([#17](https://github.com/elastio/bon/pull/17))

## [1.0.2](https://github.com/elastio/bon/compare/v1.0.1...v1.0.2) - 2024-07-29

### Fixed
- Fix a bug of the `Default` trait requirement for types under an `Option` ([#13](https://github.com/elastio/bon/pull/13))
- Fix the link to docs.rs to so that it references the latest version ([#11](https://github.com/elastio/bon/pull/11))

## [1.0.1](https://github.com/elastio/bon/compare/v1.0.0...v1.0.1) - 2024-07-29

### Fixed
- Fix handling of raw identifiers ([#9](https://github.com/elastio/bon/pull/9))

### Other
- Add example snippet to the docs for "adding builder to existing code" ([#7](https://github.com/elastio/bon/pull/7))

## [1.0.0](https://github.com/elastio/bon/tree/v1.0.0) - 2024-07-28

### Added

- Initial release 🎉. See the [`bon` crate overview for details](https://elastio.github.io/bon/guide/overview).
