# Contributing to TreeLog

Thank you for your interest in contributing to TreeLog! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

Before creating a bug report, please check existing issues to see if the problem has already been reported.

When creating a bug report, please include:
- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Environment details (Rust version, OS, etc.)
- Any relevant code snippets or examples

### Suggesting Enhancements

Enhancement suggestions are welcome! Please include:
- A clear description of the enhancement
- Use cases and examples
- Any potential implementation considerations

### Pull Requests

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** following the coding standards below
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Run tests** to ensure everything passes:
   ```bash
   cargo test --features all
   cargo test --features all --doc
   ```
6. **Commit your changes** with clear, descriptive commit messages
7. **Push to your fork** and open a pull request

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/muijf/treelog.git
   cd treelog
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test --features all
   ```

4. Run examples:
   ```bash
   cargo run --example basic
   ```

5. **Set up pre-commit hooks** (optional but recommended):
   ```bash
   # Install pre-commit
   pip install pre-commit

   # Install git hooks
   pre-commit install
   ```

   Pre-commit hooks will automatically run code formatting and linting checks before each commit. See [pre-commit.com](https://pre-commit.com/) for more information.

## Coding Standards

### Code Style

- Follow Rust standard formatting (use `cargo fmt`)
- Run `cargo clippy` and fix any warnings
- Keep functions focused and modular
- Add documentation comments for public APIs

### Testing

- Write unit tests for new functionality
- Ensure all tests pass with `cargo test --features all`
- Include doctests in documentation examples
- Test edge cases and error conditions

### Documentation

- Document all public APIs
- Include examples in doc comments
- Update README.md for user-facing changes
- Keep code comments clear and concise

### Features

- When adding new features, consider if they should be feature-gated
- Update `Cargo.toml` with new features if needed
- Document feature requirements in code

## Feature Guidelines

The library uses feature flags to allow users to opt-in to functionality:

- **Core features** (always available): Tree construction, basic rendering
- **Optional features**: `builder`, `iterator`, `formatters`, `color`

When adding new functionality:
- Consider if it should be a feature flag
- Update the `all` feature to include it
- Document it in the README

## Commit Messages

Write clear, descriptive commit messages:
- Use the imperative mood ("Add feature" not "Added feature")
- Keep the first line under 72 characters
- Include more details in the body if needed
- Reference issue numbers if applicable

Example:
```
Add support for custom tree styles

Implements StyleConfig::custom() method allowing users to define
their own character sets for tree rendering. Includes tests and
documentation updates.

Fixes #123
```

## Questions?

If you have questions about contributing, feel free to:
- Open an issue with the "question" label
- Check existing documentation
- Review existing code for examples

Thank you for contributing to TreeLog!
