# flowerpassword.rust

[![CI](https://github.com/xlsdg/flowerpassword.rust/actions/workflows/ci.yml/badge.svg)](https://github.com/xlsdg/flowerpassword.rust/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/flowerpassword.svg)](https://crates.io/crates/flowerpassword)
[![Documentation](https://docs.rs/flowerpassword/badge.svg)](https://docs.rs/flowerpassword)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Flower Password implementation for Rust - a secure password generator based on master password and key using MD5/HMAC-MD5 hashing.

## Overview

This library implements the Flower Password algorithm, which generates secure, deterministic passwords based on:

- A master password (your secret)
- A key (e.g., website domain, service name)
- A desired length (2-32 characters)

The same inputs will always generate the same output, allowing you to recreate passwords without storing them.

## Features

- 🔒 **Secure**: Uses HMAC-MD5 with multiple rounds of hashing
- 🎯 **Deterministic**: Same inputs always generate the same password
- 📏 **Flexible**: Password length from 2 to 32 characters
- ✅ **Validated**: Comprehensive test suite
- 🦀 **Pure Rust**: No unsafe code, zero dependencies (except `md5` crate)
- 🔄 **Compatible**: Produces identical output to the JavaScript implementation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
flowerpassword = "1.0"
```

## Usage

### Basic Example

```rust
use flowerpassword::fp_code;

fn main() {
    // Generate a 16-character password for GitHub
    match fp_code("my_master_password", "github.com", 16) {
        Ok(password) => println!("Password: {}", password),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Different Lengths

```rust
use flowerpassword::fp_code;

fn main() {
    let master = "my_secret_password";

    // Short password (8 characters)
    let short = fp_code(master, "example.com", 8).unwrap();

    // Medium password (16 characters)
    let medium = fp_code(master, "github.com", 16).unwrap();

    // Long password (32 characters)
    let long = fp_code(master, "bank.com", 32).unwrap();

    println!("Short:  {}", short);
    println!("Medium: {}", medium);
    println!("Long:   {}", long);
}
```

### Error Handling

```rust
use flowerpassword::{fp_code, FlowerPasswordError};

fn main() {
    // Valid range is 2-32
    match fp_code("password", "key", 1) {
        Ok(pwd) => println!("Password: {}", pwd),
        Err(FlowerPasswordError::InvalidLength(len)) => {
            eprintln!("Invalid length: {}", len);
        }
    }

    match fp_code("password", "key", 50) {
        Ok(pwd) => println!("Password: {}", pwd),
        Err(FlowerPasswordError::InvalidLength(len)) => {
            eprintln!("Invalid length: {}", len);
        }
    }
}
```

### Multiple Services

```rust
use flowerpassword::fp_code;

fn main() {
    let master = "my_master_password";

    let services = vec![
        "github.com",
        "google.com",
        "twitter.com",
        "facebook.com",
    ];

    for service in services {
        let password = fp_code(master, service, 16).unwrap();
        println!("{}: {}", service, password);
    }
}
```

## API Reference

### `fp_code`

Generates a Flower Password based on master password and key.

```rust
pub fn fp_code(
    password: &str,
    key: &str,
    length: usize
) -> Result<String, FlowerPasswordError>
```

**Parameters:**

- `password`: Master password (any string)
- `key`: Domain or service identifier (any string)
- `length`: Output password length (2-32 characters)

**Returns:**

- `Ok(String)`: Generated password
- `Err(FlowerPasswordError)`: If length is invalid

**Errors:**

- `FlowerPasswordError::InvalidLength`: Length is not between 2 and 32

## Algorithm

The Flower Password algorithm:

1. Generates HMAC-MD5 hash from password and key
2. Creates two derivative hashes using fixed salts ("kise" for rules, "snow" for source)
3. Applies character transformation rules based on a magic string
4. Ensures the first character is always alphabetic
5. Returns the password at the requested length

### Character Composition

Generated passwords contain:

- Uppercase letters (A-Z)
- Lowercase letters (a-z)
- Digits (0-9)

The first character is always a letter (never a digit).

## Testing

Run the test suite:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

Run doc tests:

```bash
cargo test --doc
```

## Examples

Example outputs for reference:

| Master Password | Key | Length | Generated Password |
|----------------|-----|--------|-------------------|
| `password` | `key` | 16 | `K3A2a66Bf88b628c` |
| `test` | `github.com` | 16 | `D04175F7A9c7Ab4a` |
| `mypassword` | `example.com` | 12 | `K0CA12CecFFB` |
| `secret` | `google.com` | 16 | `Kc6813f75AAa6Bd1` |

## Compatibility

This Rust implementation produces identical output to:

- [flowerpassword.js](https://github.com/xlsdg/flowerpassword.js) v5.0.0+

## Security Considerations

⚠️ **Important Notes:**

1. **Master Password**: Keep your master password secure. If compromised, all generated passwords are compromised.
2. **MD5 Usage**: This algorithm uses MD5 for historical compatibility. While MD5 is not recommended for cryptographic purposes, the Flower Password algorithm applies it in multiple rounds with HMAC, which provides reasonable security for password generation.
3. **Deterministic**: Outputs are deterministic - same inputs always produce same outputs. This is a feature, not a bug.
4. **No Storage**: Passwords are generated on-demand, not stored. If you forget your master password or key, you cannot recover the generated password.

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run clippy (linter)
cargo clippy -- -D warnings

# Format code
cargo fmt

# Build documentation
cargo doc --open
```

### Releasing

The project uses automated CI/CD with GitHub Actions.

**Quick release:**

```bash
# Using the release script (easiest)
./release.sh patch   # Bug fixes: 1.0.0 → 1.0.1
./release.sh minor   # New features: 1.0.0 → 1.1.0
./release.sh major   # Breaking changes: 1.0.0 → 2.0.0

# Or manually with GitHub Actions
# Go to Actions → Manual Release → Run workflow
```

The release process automatically:

- Runs all tests
- Publishes to crates.io
- Creates a GitHub release

## Author

xLsDg <xlsdg@qq.com> (<https://xlsdg.org/>)

## Links

- [GitHub Repository](https://github.com/xlsdg/flowerpassword.rust)
- [JavaScript Version](https://github.com/xlsdg/flowerpassword.js)
