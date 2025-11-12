//! Flower Password implementation for Rust
//!
//! This library implements the Flower Password algorithm, which generates secure passwords
//! based on a master password and a key using MD5/HMAC-MD5 hashing with a specific
//! transformation algorithm.
//!
//! # Example
//!
//! ```
//! use flowerpassword::fp_code;
//!
//! let password = fp_code("test", "github.com", 16).unwrap();
//! assert_eq!(password, "D04175F7A9c7Ab4a");
//! ```

use std::error::Error;
use std::fmt;

/// Minimum valid password length
const MIN_LENGTH: usize = 2;

/// Maximum valid password length
const MAX_LENGTH: usize = 32;

/// Magic string used for character transformation rules
/// This is part of the Flower Password algorithm specification
const MAGIC_STRING: &str = "sunlovesnow1990090127xykab";

/// MD5 hash length in hexadecimal characters
const MD5_HEX_LENGTH: usize = 32;

/// Error type for Flower Password operations
#[derive(Debug, Clone)]
pub enum FlowerPasswordError {
    /// Length parameter is outside the valid range
    InvalidLength(usize),
}

impl fmt::Display for FlowerPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlowerPasswordError::InvalidLength(len) => {
                write!(
                    f,
                    "Length must be between {} and {}, got: {}",
                    MIN_LENGTH, MAX_LENGTH, len
                )
            }
        }
    }
}

impl Error for FlowerPasswordError {}

/// Computes HMAC-MD5 hash
///
/// This function implements HMAC-MD5 to match the behavior of blueimp-md5's
/// two-parameter mode used in the JavaScript implementation.
///
/// Special case: When key is empty, returns regular MD5 hash (not HMAC)
/// to match blueimp-md5 behavior.
fn hmac_md5(message: &str, key: &str) -> String {
    // Special case: empty key returns regular MD5 (matching blueimp-md5 behavior)
    if key.is_empty() {
        let digest = md5::compute(message.as_bytes());
        return format!("{:x}", digest);
    }

    let key_bytes = key.as_bytes();
    let message_bytes = message.as_bytes();

    const BLOCK_SIZE: usize = 64;

    // Prepare the key
    let mut key_block = [0u8; BLOCK_SIZE];
    if key_bytes.len() > BLOCK_SIZE {
        // If key is longer than block size, hash it
        let digest = md5::compute(key_bytes);
        key_block[..16].copy_from_slice(&digest.0);
    } else {
        // Otherwise, use key as-is (padded with zeros)
        key_block[..key_bytes.len()].copy_from_slice(key_bytes);
    }

    // Create inner and outer padded keys
    let mut ipad = [0x36u8; BLOCK_SIZE];
    let mut opad = [0x5cu8; BLOCK_SIZE];

    for i in 0..BLOCK_SIZE {
        ipad[i] ^= key_block[i];
        opad[i] ^= key_block[i];
    }

    // Compute inner hash: H(K XOR ipad, message)
    let mut inner_data = Vec::with_capacity(BLOCK_SIZE + message_bytes.len());
    inner_data.extend_from_slice(&ipad);
    inner_data.extend_from_slice(message_bytes);
    let inner_hash = md5::compute(&inner_data);

    // Compute outer hash: H(K XOR opad, inner_hash)
    let mut outer_data = Vec::with_capacity(BLOCK_SIZE + 16);
    outer_data.extend_from_slice(&opad);
    outer_data.extend_from_slice(&inner_hash.0);
    let outer_hash = md5::compute(&outer_data);

    // Return as hex string
    format!("{:x}", outer_hash)
}

/// Validates the length parameter
fn validate_length(length: usize) -> Result<(), FlowerPasswordError> {
    if !(MIN_LENGTH..=MAX_LENGTH).contains(&length) {
        Err(FlowerPasswordError::InvalidLength(length))
    } else {
        Ok(())
    }
}

/// Core algorithm to generate Flower Password from MD5 hashes
fn generate_password(rule_hash: &str, source_hash: &str, length: usize) -> String {
    let rule_chars: Vec<char> = rule_hash.chars().collect();
    let mut source_chars: Vec<char> = source_hash.chars().collect();

    // Apply transformation rules: uppercase letters based on magic string pattern
    for i in 0..MD5_HEX_LENGTH {
        let ch = source_chars[i];
        // Check if character is a letter (not a digit)
        if !ch.is_ascii_digit() {
            // Check if rule character exists in magic string
            if MAGIC_STRING.contains(rule_chars[i]) {
                source_chars[i] = ch.to_ascii_uppercase();
            }
        }
    }

    let transformed_hash: String = source_chars.iter().collect();
    let first_char = transformed_hash.chars().next().unwrap();

    // Ensure first character is always a letter (replace with 'K' if it's a digit)
    let first = if first_char.is_ascii_digit() {
        'K'
    } else {
        first_char
    };

    // Build the final password
    let mut result = String::with_capacity(length);
    result.push(first);
    result.push_str(&transformed_hash[1..length]);

    result
}

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
/// # Errors
///
/// Returns `FlowerPasswordError::InvalidLength` if length is not between 2 and 32.
///
/// # Example
///
/// ```
/// use flowerpassword::fp_code;
///
/// let password = fp_code("test", "github.com", 16).unwrap();
/// assert_eq!(password, "D04175F7A9c7Ab4a");
/// ```
pub fn fp_code(password: &str, key: &str, length: usize) -> Result<String, FlowerPasswordError> {
    validate_length(length)?;

    // Generate base MD5 hash from password and key using HMAC
    let base_hash = hmac_md5(password, key);

    // Generate rule and source hashes using fixed salts
    let rule_hash = hmac_md5(&base_hash, "kise");
    let source_hash = hmac_md5(&base_hash, "snow");

    Ok(generate_password(&rule_hash, &source_hash, length))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic functionality tests
    #[test]
    fn test_generate_password_with_length_16() {
        let result = fp_code("password", "key", 16).unwrap();
        assert_eq!(result, "K3A2a66Bf88b628c");
    }

    #[test]
    fn test_consistent_passwords_for_same_inputs() {
        let result1 = fp_code("password", "key", 16).unwrap();
        let result2 = fp_code("password", "key", 16).unwrap();
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_different_passwords_for_different_passwords() {
        let result1 = fp_code("password1", "key", 16).unwrap();
        let result2 = fp_code("password2", "key", 16).unwrap();
        assert_ne!(result1, result2);
    }

    #[test]
    fn test_different_passwords_for_different_keys() {
        let result1 = fp_code("password", "key1", 16).unwrap();
        let result2 = fp_code("password", "key2", 16).unwrap();
        assert_ne!(result1, result2);
    }

    // Length validation tests
    #[test]
    fn test_length_2() {
        let result = fp_code("password", "key", 2).unwrap();
        assert_eq!(result, "K3");
    }

    #[test]
    fn test_length_8() {
        let result = fp_code("password", "key", 8).unwrap();
        assert_eq!(result, "K3A2a66B");
    }

    #[test]
    fn test_length_16() {
        let result = fp_code("password", "key", 16).unwrap();
        assert_eq!(result, "K3A2a66Bf88b628c");
    }

    #[test]
    fn test_length_24() {
        let result = fp_code("password", "key", 24).unwrap();
        assert_eq!(result, "K3A2a66Bf88b628c2Cd7cDA9");
    }

    #[test]
    fn test_length_32() {
        let result = fp_code("password", "key", 32).unwrap();
        assert_eq!(result, "K3A2a66Bf88b628c2Cd7cDA9958f6b26");
    }

    // First character requirement tests
    #[test]
    fn test_first_char_password_key() {
        let result = fp_code("password", "key", 16).unwrap();
        assert_eq!(result, "K3A2a66Bf88b628c");
        assert!(result.chars().next().unwrap().is_ascii_alphabetic());
    }

    #[test]
    fn test_first_char_test_example() {
        let result = fp_code("test", "example.com", 16).unwrap();
        assert_eq!(result, "B0399e643E07a2EA");
        assert!(result.chars().next().unwrap().is_ascii_alphabetic());
    }

    #[test]
    fn test_first_char_mypass_github() {
        let result = fp_code("mypass", "github.com", 16).unwrap();
        assert_eq!(result, "K5817EB58CE4512F");
        assert!(result.chars().next().unwrap().is_ascii_alphabetic());
    }

    #[test]
    fn test_first_char_secret_google() {
        let result = fp_code("secret", "google.com", 16).unwrap();
        assert_eq!(result, "Kc6813f75AAa6Bd1");
        assert!(result.chars().next().unwrap().is_ascii_alphabetic());
    }

    #[test]
    fn test_first_char_numeric_password() {
        let result = fp_code("12345", "site", 16).unwrap();
        assert_eq!(result, "K05a62bfea0C1553");
        assert!(result.chars().next().unwrap().is_ascii_alphabetic());
    }

    // Character composition tests
    #[test]
    fn test_alphanumeric_characters() {
        let result = fp_code("password", "key", 16).unwrap();
        assert_eq!(result, "K3A2a66Bf88b628c");
        for ch in result.chars() {
            assert!(ch.is_ascii_alphanumeric());
        }
    }

    #[test]
    fn test_mixed_case_letters() {
        let result = fp_code("password", "key", 32).unwrap();
        assert_eq!(result, "K3A2a66Bf88b628c2Cd7cDA9958f6b26");

        let has_uppercase = result.chars().any(|c| c.is_ascii_uppercase());
        let has_lowercase = result.chars().any(|c| c.is_ascii_lowercase());

        assert!(has_uppercase, "Should contain uppercase letters");
        assert!(has_lowercase, "Should contain lowercase letters");
    }

    // Real-world examples
    #[test]
    fn test_real_world_example_1() {
        let password = fp_code("password", "key", 16).unwrap();
        assert_eq!(password, "K3A2a66Bf88b628c");
    }

    #[test]
    fn test_real_world_example_2() {
        let password = fp_code("test", "github.com", 16).unwrap();
        assert_eq!(password, "D04175F7A9c7Ab4a");
    }

    #[test]
    fn test_real_world_example_3() {
        let password = fp_code("mypassword", "example.com", 12).unwrap();
        assert_eq!(password, "K0CA12CecFFB");
    }

    // Edge cases
    #[test]
    fn test_empty_password_string() {
        let result = fp_code("", "key", 16).unwrap();
        assert_eq!(result, "K46eB52c968caeAa");
    }

    #[test]
    fn test_empty_key_string() {
        let result = fp_code("password", "", 16).unwrap();
        assert_eq!(result, "eB3b1cA3D6B54c00");
    }

    #[test]
    fn test_both_empty_strings() {
        let result = fp_code("", "", 16).unwrap();
        assert_eq!(result, "K930B0264e62DDFC");
    }

    #[test]
    fn test_special_characters_in_password() {
        let result = fp_code("p@ssw0rd!#$%", "key", 16).unwrap();
        assert_eq!(result, "D4e5c2BE16F71498");
    }

    #[test]
    fn test_special_characters_in_key() {
        let result = fp_code("password", "user@example.com", 16).unwrap();
        assert_eq!(result, "K98076292B62A974");
    }

    #[test]
    fn test_unicode_characters() {
        let result = fp_code("密码", "网站.com", 16).unwrap();
        assert_eq!(result, "KFF7FEa7928bAAAa");
    }

    #[test]
    fn test_very_long_password() {
        let long_password = "a".repeat(1000);
        let result = fp_code(&long_password, "key", 16).unwrap();
        assert_eq!(result, "K2775CF7c646a718");
    }

    #[test]
    fn test_very_long_key() {
        let long_key = "b".repeat(1000);
        let result = fp_code("password", &long_key, 16).unwrap();
        assert_eq!(result, "K77E3F873Aa8a01f");
    }

    // Type safety - all valid lengths
    #[test]
    fn test_all_valid_lengths() {
        let expected: Vec<(&str, usize)> = vec![
            ("K3", 2),
            ("K3A", 3),
            ("K3A2", 4),
            ("K3A2a", 5),
            ("K3A2a6", 6),
            ("K3A2a66", 7),
            ("K3A2a66B", 8),
            ("K3A2a66Bf", 9),
            ("K3A2a66Bf8", 10),
            ("K3A2a66Bf88", 11),
            ("K3A2a66Bf88b", 12),
            ("K3A2a66Bf88b6", 13),
            ("K3A2a66Bf88b62", 14),
            ("K3A2a66Bf88b628", 15),
            ("K3A2a66Bf88b628c", 16),
            ("K3A2a66Bf88b628c2", 17),
            ("K3A2a66Bf88b628c2C", 18),
            ("K3A2a66Bf88b628c2Cd", 19),
            ("K3A2a66Bf88b628c2Cd7", 20),
            ("K3A2a66Bf88b628c2Cd7c", 21),
            ("K3A2a66Bf88b628c2Cd7cD", 22),
            ("K3A2a66Bf88b628c2Cd7cDA", 23),
            ("K3A2a66Bf88b628c2Cd7cDA9", 24),
            ("K3A2a66Bf88b628c2Cd7cDA99", 25),
            ("K3A2a66Bf88b628c2Cd7cDA995", 26),
            ("K3A2a66Bf88b628c2Cd7cDA9958", 27),
            ("K3A2a66Bf88b628c2Cd7cDA9958f", 28),
            ("K3A2a66Bf88b628c2Cd7cDA9958f6", 29),
            ("K3A2a66Bf88b628c2Cd7cDA9958f6b", 30),
            ("K3A2a66Bf88b628c2Cd7cDA9958f6b2", 31),
            ("K3A2a66Bf88b628c2Cd7cDA9958f6b26", 32),
        ];

        for (expected_pwd, length) in expected {
            let result = fp_code("password", "key", length).unwrap();
            assert_eq!(result, expected_pwd, "Failed for length {}", length);
        }
    }

    // Length validation errors
    #[test]
    fn test_error_length_0() {
        let result = fp_code("password", "key", 0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Length must be between 2 and 32, got: 0"
        );
    }

    #[test]
    fn test_error_length_1() {
        let result = fp_code("password", "key", 1);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Length must be between 2 and 32, got: 1"
        );
    }

    #[test]
    fn test_error_length_33() {
        let result = fp_code("password", "key", 33);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Length must be between 2 and 32, got: 33"
        );
    }

    #[test]
    fn test_error_length_100() {
        let result = fp_code("password", "key", 100);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Length must be between 2 and 32, got: 100"
        );
    }

    // Additional verification tests
    #[test]
    fn test_first_char_is_always_letter() {
        for i in 0..100 {
            let password = fp_code("test", &format!("site{}.com", i), 16).unwrap();
            let first_char = password.chars().next().unwrap();
            assert!(
                first_char.is_ascii_alphabetic(),
                "First character '{}' is not alphabetic in password '{}'",
                first_char,
                password
            );
        }
    }
}
