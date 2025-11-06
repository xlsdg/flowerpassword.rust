# Contributing to flowerpassword.rust

Thank you for your interest in contributing to flowerpassword.rust! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:

- **Clear title**: Describe the bug concisely
- **Description**: Detailed description of the bug
- **Steps to reproduce**: How to reproduce the behavior
- **Expected behavior**: What you expected to happen
- **Actual behavior**: What actually happened
- **Environment**: OS, Rust version, crate version
- **Code sample**: Minimal code that reproduces the issue (if applicable)

### Suggesting Enhancements

Enhancement suggestions are welcome! Please create an issue with:

- **Clear title**: Describe the enhancement
- **Motivation**: Why this enhancement would be useful
- **Description**: Detailed description of the proposed change
- **Alternatives**: Other solutions you've considered
- **Examples**: Usage examples (if applicable)

### Pull Requests

1. **Fork the repository** and create your branch from `main`:

   ```bash
   git checkout -b feature/my-feature
   # or
   git checkout -b fix/my-bugfix
   ```

2. **Make your changes**:
   - Write clear, concise commit messages
   - Follow the existing code style
   - Add tests for new functionality
   - Update documentation as needed

3. **Test your changes**:

   ```bash
   # Run all tests
   cargo test --all-features

   # Check formatting
   cargo fmt --check

   # Run clippy
   cargo clippy --all-targets --all-features -- -D warnings

   # Check documentation
   cargo doc --no-deps --all-features
   ```

4. **Update documentation**:
   - Add/update doc comments for new/changed functions
   - Update README.md if needed
   - Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format

5. **Submit your pull request**:
   - Provide a clear description of the changes
   - Reference any related issues
   - Ensure CI passes

## Development Setup

### Prerequisites

- Rust 1.70 or later
- cargo
- git

### Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/flowerpassword.rust.git
cd flowerpassword.rust

# Add upstream remote
git remote add upstream https://github.com/xlsdg/flowerpassword.rust.git

# Install dependencies
cargo build

# Run tests
cargo test
```

### Development Workflow

```bash
# Create a new branch
git checkout -b feature/my-feature

# Make changes and commit
git add .
git commit -m "feat: add new feature"

# Run tests
cargo test --all-features

# Format code
cargo fmt

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Push to your fork
git push origin feature/my-feature
```

## Coding Guidelines

### Code Style

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting (configured in `rustfmt.toml`)
- Use `cargo clippy` for linting (configured in `clippy.toml`)
- Maximum line length: 100 characters

### Naming Conventions

- Use `snake_case` for functions, variables, modules
- Use `PascalCase` for types, traits, enums
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefix private items with underscore if unused

### Documentation

- Add doc comments (`///`) for all public items
- Include examples in doc comments
- Use `//!` for module-level documentation
- Write clear, concise documentation

Example:

```rust
/// Generates a Flower Password based on master password and key
///
/// # Arguments
///
/// * `password` - Master password
/// * `key` - Domain or service identifier
/// * `length` - Output password length (2-32 characters)
///
/// # Returns
///
/// Returns `Ok(String)` with the generated password, or `Err(FlowerPasswordError)`
/// if the length is invalid.
///
/// # Examples
///
/// ```
/// use flowerpassword::fp_code;
///
/// let password = fp_code("test", "github.com", 16).unwrap();
/// assert_eq!(password, "D04175F7A9c7Ab4a");
/// ```
pub fn fp_code(password: &str, key: &str, length: usize) -> Result<String, FlowerPasswordError> {
    // ...
}
```

### Testing

- Write unit tests for all functions
- Write integration tests for public APIs
- Use doc tests for examples
- Aim for high test coverage
- Test edge cases and error conditions

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = fp_code("password", "key", 16).unwrap();
        assert_eq!(result, "K3A2a66Bf88b628c");
    }

    #[test]
    fn test_error_handling() {
        let result = fp_code("password", "key", 1);
        assert!(result.is_err());
    }
}
```

### Error Handling

- Use `Result` for fallible operations
- Create custom error types when appropriate
- Provide descriptive error messages
- Don't panic in library code (except for unrecoverable errors)

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

Examples:

```bash
git commit -m "feat: add support for custom salt values"
git commit -m "fix: resolve panic on empty string input"
git commit -m "docs: update README with new examples"
git commit -m "test: add tests for unicode handling"
```

## Compatibility

### Algorithm Compatibility

This implementation must maintain **100% compatibility** with the JavaScript version ([flowerpassword.js](https://github.com/xlsdg/flowerpassword.js)).

**Before submitting changes that affect the algorithm:**

1. Run the compatibility test:

   ```bash
   cargo run --example compatibility_test
   ```

2. Compare outputs with JavaScript version:

   ```bash
   # In JavaScript project
   node -e "const { fpCode } = require('./dist/flowerPassword.js'); \
     console.log(fpCode('password', 'key', 16));"

   # In Rust project
   cargo run --quiet --example compatibility_test
   ```

3. Ensure all outputs match exactly

### Breaking Changes

- Avoid breaking changes when possible
- If breaking changes are necessary:
  - Document in CHANGELOG.md under `[BREAKING]`
  - Provide migration guide
  - Bump major version
  - Discuss in issue/PR first

## Release Process

Releases are automated via GitHub Actions.

Maintainers can release using:

```bash
./release.sh patch   # Bug fixes
./release.sh minor   # New features
./release.sh major   # Breaking changes
```

## Questions?

- Open an issue for questions
- Tag with `question` label
- Be respectful and patient

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing! 🎉
