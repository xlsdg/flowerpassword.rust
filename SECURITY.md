# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.x.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability, please send an email to **<xlsdg@qq.com>** with:

- **Subject**: "Security Vulnerability in flowerpassword.rust"
- **Description**: Detailed description of the vulnerability
- **Impact**: Potential impact and attack scenario
- **Reproduction**: Steps to reproduce the vulnerability
- **Suggested fix**: If you have one (optional)

### What to expect

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix timeline**: Depends on severity
  - Critical: Within 7 days
  - High: Within 14 days
  - Medium: Within 30 days
  - Low: Next planned release

### Disclosure Policy

- We follow [coordinated disclosure](https://en.wikipedia.org/wiki/Coordinated_vulnerability_disclosure)
- We will notify you when the vulnerability is fixed
- We will credit you in the release notes (unless you prefer to remain anonymous)
- Please allow us reasonable time to fix the issue before public disclosure

## Security Considerations

### Algorithm Security

This library implements the Flower Password algorithm using MD5 and HMAC-MD5:

⚠️ **Important Notes:**

1. **MD5 is not cryptographically secure**: MD5 is considered broken for cryptographic purposes. However, this library uses it for historical compatibility with the original Flower Password algorithm.

2. **HMAC-MD5 provides reasonable security**: While MD5 alone is weak, HMAC-MD5 with multiple rounds provides reasonable security for password generation purposes.

3. **Not for high-security applications**: This algorithm is suitable for general password management but may not be appropriate for high-security applications.

### Best Practices

When using this library:

✅ **Do:**

- Use a strong, unique master password
- Use different keys for different services
- Keep your master password secure and private
- Consider using a password manager for the master password

❌ **Don't:**

- Don't use weak master passwords (e.g., "password", "12345")
- Don't share your master password
- Don't use the same master password across multiple password managers
- Don't rely solely on this for critical systems

### Threat Model

**What this protects against:**

- Breach of password storage (passwords are generated, not stored)
- Different passwords for different services
- Password reuse across services

**What this does NOT protect against:**

- Weak master password (brute force attacks)
- Keyloggers or malware on your device
- Phishing attacks (you still need to verify the site)
- Quantum computing attacks (MD5 is vulnerable)

### Dependencies

This library has minimal dependencies:

- `md5` crate (v0.7) - For MD5 hashing

We monitor dependencies for security vulnerabilities using:

- Dependabot (automated updates)
- `cargo audit` (security advisories)

### Building from Source

To verify the integrity of the source code:

```bash
# Clone the repository
git clone https://github.com/xlsdg/flowerpassword.rust.git
cd flowerpassword.rust

# Verify the commit signatures (if available)
git log --show-signature

# Build from source
cargo build --release

# Run tests
cargo test --all-features
```

### Reporting Non-Security Issues

For non-security bugs and issues, please use the [GitHub Issues](https://github.com/xlsdg/flowerpassword.rust/issues) page.

## Security Updates

Security updates will be released as:

- Patch versions for non-breaking fixes
- Documented in CHANGELOG.md with `[SECURITY]` tag
- Announced in GitHub releases

Subscribe to GitHub releases to receive notifications.

## Compliance

This project:

- ✅ Uses standard Rust security practices
- ✅ Follows OWASP guidelines where applicable
- ✅ Undergoes automated security scanning (Dependabot)
- ✅ Maintains minimal dependencies

## References

- [OWASP Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [Rust Security Working Group](https://www.rust-lang.org/governance/wgs/wg-security-response)
- [RustSec Advisory Database](https://rustsec.org/)
- [Original Flower Password Algorithm](https://github.com/xlsdg/flowerpassword.js)

## Contact

For security concerns: **<xlsdg@qq.com>**

For general questions: Use [GitHub Issues](https://github.com/xlsdg/flowerpassword.rust/issues)

---

Thank you for helping keep flowerpassword.rust secure!
