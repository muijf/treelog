# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.6] - 2025-11-27

### Changed
- Reorganized feature flags: renamed `comparison` to `compare`
- Added new feature flags: `search`, `sort`, `stats`
- Reorganized arbitrary features: `walkdir`, `petgraph`, `cargo-metadata`, `git2`, `syn`, `tree-sitter`, and `clap` now use `arbitrary-*` prefix
- Updated example required-features to use new feature names

### Fixed
- Updated README.md documentation

## [0.0.5] - 2025-11-27

### Added
- CLI binary with command-line interface support
- Serde integration support (JSON, YAML, TOML, RON serialization/deserialization)
- Walkdir integration for file system tree building from directories
- Petgraph integration for graph to/from tree conversion
- Cargo metadata integration for Cargo dependency tree visualization
- Git2 integration for Git repository structure visualization
- XML/HTML DOM tree visualization via roxmltree
- Rust AST visualization via syn
- Tree-sitter parse tree visualization
- Clap command-line argument structure visualization

### Fixed
- HTML export output formatting
- SVG export functionality

### Changed
- Added feature flags to OutputFormat
- Reorganized project structure

## [0.0.4] - 2025-11-26

### Added
- Macro DSL for tree construction
- LevelPath newtype for tree path utilities
- Comparison operations for tree diff
- Export functionality (HTML, SVG, DOT formats)
- Merge operations with different strategies
- Path utilities (get by path, flatten)
- Transform operations (map, filter, prune)
- Traversal iterators (pre-order, post-order, level-order)
- Extra tree statistics

### Changed
- Updated README.md documentation
- Improved project structure

### Fixed
- Removed temporary files from repository
- Fixed random exports included in git

## [0.0.3] - 2025-11-26

### Added
- CI workflow with automated checks
- README.md shields and badges
- Version validation script
- Publish workflow for automated releases
- Changelog file
- FUNDING.yml for GitHub Sponsors
- VS Code settings for development
- Project banner

### Changed
- More concise README.md
- Improved documentation

### Fixed
- Fixed all compiler warnings
- Fixed formatting errors
- Fixed clippy lints

## [0.0.2] - 2025-11-26

### Added
- Initial release with core tree rendering functionality
- Multiple style options (Unicode, ASCII, Box drawing, custom)
- Builder API for fluent tree construction
- Iterator support for streaming large trees
- Custom formatters for nodes and leaves
- Feature flags for optional functionality

## [0.0.1] - 2025-11-26

### Added
- Initial project setup
- Core tree data structure
- Basic tree rendering functionality

[Unreleased]: https://github.com/muijf/treelog/compare/v0.0.6...HEAD
[0.0.6]: https://github.com/muijf/treelog/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/muijf/treelog/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/muijf/treelog/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/muijf/treelog/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/muijf/treelog/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/muijf/treelog/releases/tag/v0.0.1
