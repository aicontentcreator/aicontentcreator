use std::io::{self, Write};

use thiserror::Error;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2, ParamsBuilder,
};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305,
    aead::generic_array::{GenericArray, typenum::Unsigned},
};

const MIN_PASSWORD_LENGTH: usize = 32;

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionError(String),
    #[error("Decryption failed: {0}")]
    DecryptionError(String),
    #[error("Password hashing failed: {0}")]
    PasswordHashError(String),
    #[error("Invalid password: {0}")]
    PasswordError(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

/// Configuration for Argon2id
fn create_argon2_config() -> Argon2<'static> {
    let params = ParamsBuilder::new()
        .m_cost(64 * 1024)      // 64MB of memory
        .t_cost(3)              // 3 iterations
        .p_cost(4)              // 4 degrees of parallelism
        .output_len(32)         // Output length suitable for ChaCha20Poly1305
        .build()
        .unwrap();
    
    Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params
    )
}

/// Prompts the user for a password and returns its Argon2id hash
pub fn ask_user_for_password() -> Result<Vec<u8>, EncryptionError> {
    loop {
        print!("Enter a password: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let password = input.trim();
        
        if password.len() < MIN_PASSWORD_LENGTH {
            println!("Password must be at least {} characters.", MIN_PASSWORD_LENGTH);
            continue;
        }

        return hash_password(password.as_bytes());
    }
}

/// Computes the Argon2id hash of the input
fn hash_password(input: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);
    
    // Create Argon2id configuration
    let argon2 = create_argon2_config();

    // Hash the password
    let password_hash = argon2
        .hash_password(input, &salt)
        .map_err(|e| EncryptionError::PasswordHashError(e.to_string()))?;

    // Extract the hash bytes
    Ok(password_hash
        .hash
        .ok_or_else(|| EncryptionError::PasswordHashError("No hash value present".to_string()))?
        .as_bytes()
        .to_vec())
}

/// Encrypts the given text using ChaCha20Poly1305
pub fn encrypt(cleartext: &str, key: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, cleartext.as_bytes())
        .map_err(|e| EncryptionError::EncryptionError(e.to_string()))?;

    // Combine nonce and ciphertext
    let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
    result.extend_from_slice(&nonce);
    result.extend_from_slice(&ciphertext);
    
    Ok(result)
}

/// Decrypts the given ciphertext using ChaCha20Poly1305
pub fn decrypt(ciphertext: &[u8], key: &[u8]) -> Result<String, EncryptionError> {
    let cipher = ChaCha20Poly1305::new(GenericArray::from_slice(key));
    
    type NonceSize = <ChaCha20Poly1305 as AeadCore>::NonceSize;
    let nonce_size = NonceSize::to_usize();
    
    if ciphertext.len() < nonce_size {
        return Err(EncryptionError::DecryptionError("Ciphertext too short".to_string()));
    }

    let (nonce, encrypted_data) = ciphertext.split_at(nonce_size);
    let nonce = GenericArray::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce, encrypted_data)
        .map_err(|e| EncryptionError::DecryptionError(e.to_string()))?;

    String::from_utf8(plaintext)
        .map_err(EncryptionError::from)
}