# Justfile for flowerpassword.rust
# https://github.com/casey/just
#
# Install just: cargo install just
# Run: just <recipe>

# Default recipe (list all available recipes)
default:
    @just --list

# Run all checks (format, clippy, test)
check: fmt clippy test doc

# Format code
fmt:
    cargo fmt --all

# Check code formatting without modifying files
fmt-check:
    cargo fmt --all -- --check

# Run clippy linter
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run clippy with fixes
clippy-fix:
    cargo clippy --all-targets --all-features --fix

# Run all tests
test:
    cargo test --all-features --verbose

# Run tests with output
test-output:
    cargo test --all-features -- --nocapture

# Run tests for a specific test name
test-name name:
    cargo test {{name}} -- --nocapture

# Run doc tests only
test-doc:
    cargo test --doc --all-features

# Run examples
example name:
    cargo run --example {{name}}

# Run all examples
examples:
    @echo "Running basic example..."
    @cargo run --example basic
    @echo "\nRunning compatibility test..."
    @cargo run --example compatibility_test

# Build the project
build:
    cargo build

# Build with release optimizations
build-release:
    cargo build --release

# Generate documentation
doc:
    cargo doc --no-deps --all-features

# Generate and open documentation
doc-open:
    cargo doc --no-deps --all-features --open

# Clean build artifacts
clean:
    cargo clean

# Update dependencies
update:
    cargo update

# Check for outdated dependencies
outdated:
    cargo outdated

# Audit dependencies for security vulnerabilities
audit:
    cargo audit

# Install development tools
install-tools:
    cargo install cargo-edit cargo-outdated cargo-audit cargo-tarpaulin

# Run code coverage
coverage:
    cargo tarpaulin --verbose --all-features --workspace --timeout 120 --out Html

# Benchmark (if benchmarks exist)
bench:
    cargo bench

# Run the release script (patch version)
release-patch:
    ./release.sh patch

# Run the release script (minor version)
release-minor:
    ./release.sh minor

# Run the release script (major version)
release-major:
    ./release.sh major

# Watch and run tests on file changes (requires cargo-watch)
watch:
    cargo watch -x test

# Watch and run checks on file changes
watch-check:
    cargo watch -x check

# Verify the project is ready for release
verify: clean check examples
    @echo "✅ All checks passed! Ready for release."

# Pre-commit checks (run before committing)
pre-commit: fmt clippy test
    @echo "✅ Pre-commit checks passed!"

# Publish to crates.io (use with caution!)
publish: verify
    cargo publish

# Publish dry run
publish-dry:
    cargo publish --dry-run
