# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

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

[unreleased]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.5...master
[v0.0.5]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.4...v0.0.5
[v0.0.4]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.3...v0.0.4
[v0.0.3]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.2...v0.0.3
[v0.0.2]: https://github.com/AndreiPashkin/compose-idents/compare/v0.0.1...v0.0.2
[v0.0.1]: https://github.com/AndreiPashkin/compose-idents/compare/1e27315fc2d46c7b61700adcf3bf4f22ea82e8e1...v0.0.1
