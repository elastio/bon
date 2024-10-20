# Changelog
All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

All the breaking changes are very unlikely to actually break your code that was written against the `v2` version of `bon` unless you've been doing some crimes like using items marked as `#[doc(hidden)]` or using unconventional macro delimiters like `#[builder{}/[]]` instead of `#[builder()]`. See also the "Removed" section about removed/replaced deprecated APIs that you most likely never used.

### Changed

- Reject unnecessary empty attributes e.g. `#[builder()]` or `#[builder]` with no parameters on a member ([#145](https://github.com/elastio/bon/pull/145))
- Reject non-empty `#[bon(...)]` attribute. This attribute will accept some parameters in future releases ([#145](https://github.com/elastio/bon/pull/145))
- Reject square brackets and curly braces delimiters for `builder_type`, `finish_fn`, `start_fn` and `on` attributes syntax. Only parentheses are accepted e.g. `#[builder(finish_fn(...))]` or `#[builder(on(...))]`. This no longer works: `#[builder(finish_fn[...])]` or `#[builder(on{...})]` ([#145](https://github.com/elastio/bon/pull/145))
- `#[builder(derive(Clone, Debug))]` now generates impl blocks that follow the behavior of standard `Clone` and `Debug` derives in that it conservatively adds `Clone/Debug` trait bounds for all the generic types declared on the original item (struct or function). See the *Added* section for details on the way to override these bounds with `#[builder(derive(Clone/Debug(bounds(...))))]`.

### Removed
- Removed support for `#[bon::builder]` proc-macro attribute on top of a `struct`. Use `#[derive(bon::Builder)]` for that instead. This syntax has been deprecated since `2.1` and it is now removed as part of a major version cleanup ([#145](https://github.com/elastio/bon/pull/145))

### Added

- Add `#[builder(builder_type(vis = "..."))]` that allows overriding the visibility of the builder struct ([#145](https://github.com/elastio/bon/pull/145))
- Add `#[builder(finish_fn(vis = "..."))]` that allows overriding the visibility of the finishing function ([#145](https://github.com/elastio/bon/pull/145))
- Add `#[builder(with = closure)]` syntax to customize setters with a custom closure. If the closure returns a `Result<_, E>` the setters become fallible ([#145](https://github.com/elastio/bon/pull/145))
- Add `#[builder(transparent)]` for `Option` fields to opt out from their special handling which makes `bon` treat them as regular required fields ([#145](https://github.com/elastio/bon/pull/145))
- Add `#[builder(state_mod)]` to configure the builder's type state API module name, visibility and docs ([#145](https://github.com/elastio/bon/pull/145))
- Add `#[builder(overwritable)]` and `#[builder(on(..., overwritable)]` to make it possible to call setters multiple times for the same member ([#145](https://github.com/elastio/bon/pull/145))
- Add `#[builder(setters)]` to fine-tune the setters names, visibility and docs ([#145](https://github.com/elastio/bon/pull/145))
- Add `#[builder(derive(Clone/Debug(bounds(...))]` to allow overriding trait bounds on the `Clone/Debug` impl block of the builder ([#145](https://github.com/elastio/bon/pull/145))
- Improve rustdoc output ([#145](https://github.com/elastio/bon/pull/145))
  - Add info that the member is required or optional.
  - For members with defaults values show the default value in the docs.
  - For optional members provide a link to a companion setter. The docs for `{member}(T)` setter mention the `maybe_{member}(Option<T>)` setter and vice versa.
  - Remove `__` prefixes for generic types and lifetimes from internal symbols. Instead, the prefixes added only if the macro detects a name collision.
- Add inheritance of `#[allow()]` and `#[expect()]` lint attributes to all generated items. This is useful to suppress any lints coming from the generated code. Although, lints coming from the generated code are generally considered defects in `bon` and should be reported via a Github issue but this provides an easy temporary workaround the problem ([#145](https://github.com/elastio/bon/pull/145))

### Fixed

- Fixed `#[cfg/cfg_attr()]` not being expanded when used on function arguments with doc comments or other attributes.


### Other

- Added graceful internal panic handling. If some `bon` macro panics due to an internal bug, the macro will try to still generate a fallback for IDEs to still provide intellisense ([#145](https://github.com/elastio/bon/pull/145))

## [2.3.0](https://github.com/elastio/bon/compare/v2.2.1...v2.3.0) - 2024-09-14

See the [blog post for this release](https://elastio.github.io/bon/blog/bon-builder-v2-3-release) that describes some of the most notable changes in detail.

### Added

- Add support for positional params in `start_fn` and `finish_fn` ([#125](https://github.com/elastio/bon/pull/125))

### Fixed

- Forward lint suppression from `#[allow()/expect()]` attributes written by the user on the top-level to the generated items ([#125](https://github.com/elastio/bon/pull/125))
- Suppress the false-positive ([clippy issue](https://github.com/rust-lang/rust-clippy/issues/6947)) `clippy::future_not_send` lint ([#125](https://github.com/elastio/bon/pull/125))
- Fix the cases where `#[builder(derive(Debug, Clone))]` didn't validate for all members to implement `Clone/Debug` if these members were of reference or generic types ([#125](https://github.com/elastio/bon/pull/125))

## [2.2.1](https://github.com/elastio/bon/compare/v2.2.0...v2.2.1) - 2024-09-09

### Changed

- Lower MSRV from 1.70.0 to 1.59.0 ([#120](https://github.com/elastio/bon/pull/120))

## [2.2.0](https://github.com/elastio/bon/compare/v2.1.1...v2.2.0) - 2024-09-08

See the [blog post for this release](https://elastio.github.io/bon/blog/bon-builder-v2-2-release) that describes some of the most notable changes in detail.

### Changed

- The `#[bon::builder]` attribute was deprecated on structs. The new [`#[derive(bon::Builder)]`](https://elastio.github.io/bon/reference/builder) should be used to derive a builder from a struct. Starting with `bon` 2.3 (next minor release) all usages of `#[bon::builder]` on structs will generate deprecation warnings. ([#99](https://github.com/elastio/bon/pull/99)).

  There is a CLI to assist in migrating to the new syntax. See the [release blog post](https://elastio.github.io/bon/blog/bon-builder-v2-2-release#derive-builder-syntax-for-structs) for details about that.

### Added

- Add the top-level `#[builder(derive(...))]` attribute to be able to derive `Clone` and `Debug` for the builder type itself ([#113](https://github.com/elastio/bon/pull/113))

- Add support for conditional compilation with `cfg/cfg_attr` ([#99](https://github.com/elastio/bon/pull/99))

### Fixed

- Fix developer experience in Rust Rover. The new `#[derive(Builder)]` syntax should now be easier for Rust Rover to analyze ([#99](https://github.com/elastio/bon/pull/99))
- Fix a bug where a member of opaque `Option` type (i.e. the `Option` type that was renamed to make the builder macro not detect it as `Option`) was still optional. ([#99](https://github.com/elastio/bon/pull/99))
- Fix code generation for structs with default values for generic parameters ([#108](https://github.com/elastio/bon/pull/108))

## [2.1.1](https://github.com/elastio/bon/compare/v2.1.0...v2.1.1) - 2024-09-03

### Added

- Set MSRV to 1.70.0. Note that we plan to set an even lower MSRV. This is just an initial attempt to define the MSRV that should be good enough in the meantime while we work on lowering it even more ([#101](https://github.com/elastio/bon/pull/101))

### Fixed

- Fix lints triggered by generated code such as `private_bounds`, `clippy::missing_const_for_fn` ([#101](https://github.com/elastio/bon/pull/101))
- Add more context to the messages such that it's clear what member isn't set in Rust Analyzer error messages ([#98](https://github.com/elastio/bon/pull/98))

## [2.1.0](https://github.com/elastio/bon/compare/v2.0.1...v2.1.0) - 2024-09-01

See the [blog post for this release](https://elastio.github.io/bon/blog/bon-builder-v2-1-release) that describes some of the most notable changes in detail.

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

### Fixed

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
