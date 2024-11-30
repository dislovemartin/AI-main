
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use std::error::Error;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

/// Encrypts data using AES-256-CBC.
///
/// # Arguments
/// * `data` - The plaintext data to encrypt.
/// * `key` - A 32-byte key for AES-256 encryption.
///
/// # Returns
/// A `Result` containing the encrypted data with a prepended IV or an error message.
pub fn encrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn Error>> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes.".into());
    }
    if data.is_empty() {
        return Err("Data cannot be empty.".into());
    }

    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let cipher = Aes256Cbc::new_from_slices(key, &iv)?;
    let ciphertext = cipher.encrypt_vec(data);

    Ok([iv.as_ref(), ciphertext.as_ref()].concat())
}

/// Decrypts data using AES-256-CBC.
///
/// # Arguments
/// * `data` - The encrypted data, which includes the IV as the first 16 bytes.
/// * `key` - A 32-byte key for AES-256 decryption.
///
/// # Returns
/// A `Result` containing the decrypted plaintext data or an error message.
pub fn decrypt(data: &[u8], key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn Error>> {
    if key.len() != 32 {
        return Err("Key must be 32 bytes.".into());
    }
    if data.len() <= 16 {
        return Err("Encrypted data must include an IV (16 bytes) and ciphertext.".into());
    }

    let (iv, ciphertext) = data.split_at(16);
    let cipher = Aes256Cbc::new_from_slices(key, iv)?;
    Ok(cipher.decrypt_vec(ciphertext)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_valid() {
        let key = [0u8; 32];
        let data = b"Sensitive data";

        let encrypted = encrypt(data, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_encrypt_invalid_key() {
        let data = b"Sensitive data";
        let invalid_key = [0u8; 16]; // Incorrect key length

        let result = encrypt(data, &invalid_key.try_into().unwrap_or([0u8; 32]));
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_invalid_data() {
        let key = [0u8; 32];
        let invalid_data = b"Short data";

        let result = decrypt(invalid_data, &key);
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_invalid_key() {
        let key = [0u8; 32];
        let data = b"Sensitive data";
        let encrypted = encrypt(data, &key).unwrap();

        let invalid_key = [1u8; 32]; // Different key
        let result = decrypt(&encrypted, &invalid_key);
        assert!(result.is_err());
    }
}
