# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial implementation of Flower Password algorithm in Rust
- HMAC-MD5 implementation matching blueimp-md5 behavior
- Comprehensive test suite (33 unit tests + 2 doc tests)
- Support for password lengths 2-32 characters
- Custom error type with descriptive messages
- Examples demonstrating usage
- Full documentation with inline examples

### Compatibility

- 100% compatible with flowerpassword.js v5.0.0+
- Produces identical output for all test cases

## [1.0.0] - YYYY-MM-DD

### Added

- First stable release
- Core `fp_code` function for password generation
- `FlowerPasswordError` for error handling
- Character transformation using magic string algorithm
- Guarantee first character is always alphabetic
- Support for empty strings, special characters, and unicode
- Comprehensive README with usage examples
- CI/CD workflows for automated testing and releasing

### Security

- Uses MD5 for historical compatibility (not recommended for cryptographic purposes)
- Multiple rounds of HMAC-MD5 provide reasonable security for password generation

---

## Release Types

- **Major**: Breaking changes, incompatible API changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes, backward compatible

## Categories

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security-related changes
