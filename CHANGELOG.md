# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

### Added

- Deprecation warning mechanism has been added. Now if a user uses something that is deprecated, the macro
  will try to attach `#[deprecated(...)]` attribute to the generated code if it's possible.

### Changed

### Removed

## [v0.1.1] - 2025-05-22

### Added

### Changed

- Make argument parsing more robust. In particular make `compose_idents!` not fail with arguments such as
  `normalize(Foo::Bar)`, where `Foo::Bar` is an enum variant or anything else that could be ambiguously interpreted
  if not parsed until the end of the token (for example `Foo::Bar` could be interpreted as an ident `Foo` and `::Bar`
  as a next completely different token).

### Removed

## [v0.1.0] - 2025-05-19

### Added

- Explicitly restricted defining duplicate aliases.
- Made it possible to re-use previously defined aliases as parts of definitions of subsequent aliases.
- Introduce the `normalize()` function, which transforms arbitrary token sequences into valid identifiers.
  Makes it possible to use things like `&'static str` in identifiers.

### Changed

- Made it possible to pass arbitrary token sequences as arguments.

### Removed

## [v0.0.7] - 2025-04-22

### Added

### Changed

- Fixed a critical bug - incorrectly configured feature flags of "syn" dependency.

### Removed

## [v0.0.6] - 2025-04-20

### Added

- New `pascal_case()` function.

### Changed

### Removed

## [v0.0.5] - 2025-04-20

### Added

### Changed

- Semicolon as the argument separator has been replaced with comma. Semicolon support
  has been preserved for backwards-compatibility.
- Fixed edge case bugs in the `snake_case` and `camel_case` functions.

### Removed

## [v0.0.4] - 2025-03-21

### Added

- Documented functions and add more clarity to the docs in general.
- Updated syn version to 2.
- Fixed handling of trailing semicolons in the macro.
- Introduced string formatting with `%alias%` syntax useful for generating doc-attributes.

### Changed

### Removed

## [v0.0.3] - 2025-02-11

### Added

- Docs/tests fixes.
- Added "functions" functionality that allows to apply functions over arguments.
- Made it possible to pass integers as arguments.
- Added "upper()", "lower()", "snake_case()" and "camel_case()" functions for case-manipulation.
- Added "hash()" function that hashes an input value deterministically within the scope
  of a single macro invocation.

### Changed

### Removed

## [v0.0.2] - 2025-02-05

### Added
- New tests.
- Support for specifying types as parts for the composed identifiers.
- Crates.io/Docs.rs badges to the README.md.

### Changed

### Removed

## [v0.0.1] - 2025-01-22

### Added

- README.md with the roadmap.
- .gitignore file.
- pre-commit config.
- Task config.
- Basic implementation of the compose-idents macro.
- GitHub CI setup.

### Changed

### Removed

[unreleased]: https://github.com/AndreiPashkin/compose-idents/compare/v0.1.1...master
[v0.1.1]: https://github.com/AndreiPashkin/compose-idents/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.7...v0.1.0
[v0.0.7]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.6...v0.0.7
[v0.0.6]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.5...v0.0.6
[v0.0.5]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.4...v0.0.5
[v0.0.4]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.3...v0.0.4
[v0.0.3]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.2...v0.0.3
[v0.0.2]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.1...v0.0.2
[v0.0.1]: https://github.com/AndreiPashkin/compose-idents/compare/1e27315fc2d46c7b61700adcf3bf4f22ea82e8e1...v0.0.1
