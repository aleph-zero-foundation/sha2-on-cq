use sha2::Digest;

use crate::types::{bytes_to_words, Word};

pub mod constants;
pub mod ops;

/// Convert `input` into an array of 16 words, which defines message schedule.
pub fn prepare_input(input: &str) -> [Word; 16] {
    let mut bytes = input.as_bytes().to_vec();

    let len = bytes.len() as u64;
    assert!(
        len < 55,
        "input is too long, we support a single message chunk at the moment"
    );

    bytes.push(0b10000000);
    while bytes.len() % 64 != 56 {
        bytes.push(0);
    }

    let len = len << 3;
    bytes.extend_from_slice(&len.to_be_bytes());

    bytes_to_words(&bytes)
}

pub fn sha_hash(input: &str) -> [Word; 8] {
    let hash = sha2::Sha256::digest(input.as_bytes()).to_vec();
    bytes_to_words(&hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_empty_input() {
        const EXPECTED: [Word; 16] = [
            0b10000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
        ];

        assert_eq!(prepare_input(""), EXPECTED);
    }

    #[test]
    fn test_sha_hash_empty_input() {
        const EXPECTED: [Word; 8] = [
            0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924, 0x27ae41e4, 0x649b934c, 0xa495991b,
            0x7852b855,
        ];
        assert_eq!(sha_hash(""), EXPECTED);
    }

    #[test]
    fn test_prepare_input_hello_world() {
        const EXPECTED: [Word; 16] = [
            0b01101000011001010110110001101100,
            0b01101111001000000111011101101111,
            0b01110010011011000110010010000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000000000000,
            0b00000000000000000000000001011000,
        ];
        assert_eq!(prepare_input("hello world"), EXPECTED);
    }

    #[test]
    fn test_sha_hash_hello_world() {
        const EXPECTED: [Word; 8] = [
            0xb94d27b9, 0x934d3e08, 0xa52e52d7, 0xda7dabfa, 0xc484efe3, 0x7a5380ee, 0x9088f7ac,
            0xe2efcde9,
        ];
        assert_eq!(sha_hash("hello world"), EXPECTED);
    }
}
