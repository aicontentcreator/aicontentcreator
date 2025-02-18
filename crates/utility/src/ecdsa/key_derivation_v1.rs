
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2;
use sha2::Sha512;
use secp256k1::{SecretKey, PublicKey, Secp256k1, Scalar};
use secp256k1::constants::SECRET_KEY_SIZE;
use zeroize::Zeroize;
use thiserror::Error;

pub const HARDENED_OFFSET: u32 = 0x80000000;
const PBKDF2_ITERATIONS: u32 = 2000;//310_000;//TOFINALIZE
const SALT: &[u8] = b"crypto_wallet_salt";

/// Custom error type for key derivation
#[derive(Debug, Error)]
pub enum KeyDerivationError {
    #[error("Seed must be exactly 64 bytes")]
    InvalidSeedLength,
    #[error("HMAC initialization failed")]
    HmacInitFailed,
    #[error("Invalid secret key derived")]
    InvalidSecretKey,
    #[error("Hardened derivation requires index >= 0x80000000")]
    InvalidHardenedIndex,
    #[error("Invalid tweak value")]
    InvalidTweakValue,
    #[error("Invalid resulting secret key")]
    InvalidResultingSecretKey,
    #[error("Cannot derive hardened key from public key")]
    PublicKeyHardenedDerivationError,
    #[error("Invalid tweak size")]
    InvalidTweakSize,
    #[error("Invalid resulting public key")]
    InvalidResultingPublicKey,
}

/// Represents an extended secret key with a chain code.
#[derive(Debug,Clone)]
pub struct ExtendedSecretKey {
    secret_key: [u8; SECRET_KEY_SIZE],
    chain_code: [u8; 32],
}

impl ExtendedSecretKey {
    /// Creates a new `ExtendedSecretKey`.
    fn new(secret_key: [u8; SECRET_KEY_SIZE], chain_code: [u8; 32]) -> Self {
        Self { secret_key, chain_code }
    }
    pub fn secret_key(&self) -> [u8; SECRET_KEY_SIZE] {
        self.secret_key
    }
}

impl Zeroize for ExtendedSecretKey {
    fn zeroize(&mut self) {
        self.secret_key.zeroize();
        self.chain_code.zeroize();
    }
}

impl Drop for ExtendedSecretKey {
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// Derives the master extended secret key using HMAC-SHA3-512.
/// 
/// # Arguments
/// - `seed`: A secure random seed (16 to 64 bytes).
pub fn derive_master_extended_secret_key(seed: &str) -> Result<ExtendedSecretKey, KeyDerivationError> {

    let mut master_seed_bytes = [0u8; 64]; 
    pbkdf2::<Hmac<Sha512>>(
        seed.as_bytes(),
        SALT,
        PBKDF2_ITERATIONS,
        &mut master_seed_bytes,
    );


    if master_seed_bytes.len() != 64 {
        //return Err("Seed must be exactly 64 bytes");
        return Err(KeyDerivationError::InvalidSeedLength);
    }



    let mut mac = Hmac::<Sha512>::new_from_slice(b"Crypto seed")
        //.map_err(|_| "HMAC initialization failed")?;
        .map_err(|_| KeyDerivationError::HmacInitFailed)?;
    mac.update(&master_seed_bytes);
    let result = mac.finalize().into_bytes();

    let (secret_key_bytes, chain_code_bytes) = result.split_at(SECRET_KEY_SIZE);

    let mut secret_key = [0u8; SECRET_KEY_SIZE];
    let mut chain_code = [0u8; 32];

    secret_key.copy_from_slice(secret_key_bytes);
    chain_code.copy_from_slice(chain_code_bytes);

    // Validate the derived secret key
    SecretKey::from_slice(&secret_key).map_err(|_| {
        secret_key.zeroize();
        chain_code.zeroize();
        //"Invalid secret key derived"
        KeyDerivationError::InvalidSecretKey
    })?;

    Ok(ExtendedSecretKey::new(secret_key, chain_code))
}

/// Derives a child extended secret key.
/// 
/// # Arguments
/// - `parent`: The parent extended secret key.
/// - `index`: The index of the child key.
/// - `hardened`: Whether the derivation is hardened.
pub fn derive_child_extended_secret_key(
    parent: &ExtendedSecretKey,
    index: u32,
    hardened: bool,
) -> Result<ExtendedSecretKey, KeyDerivationError> {
    if hardened && index < HARDENED_OFFSET {
        //return Err("Hardened derivation requires index >= 0x80000000");
        return Err(KeyDerivationError::InvalidHardenedIndex);
    }

    let mut mac = Hmac::<Sha512>::new_from_slice(&parent.chain_code)
        //.map_err(|_| "HMAC initialization failed")?;
        .map_err(|_| KeyDerivationError::HmacInitFailed)?;

    if hardened {
        mac.update(&[0x00]);
        mac.update(&parent.secret_key);
    } else {
        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&parent.secret_key)
            //.map_err(|_| "Invalid parent secret key")?;
            .map_err(|_| KeyDerivationError::InvalidSecretKey)?;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        mac.update(&public_key.serialize());
    }

    mac.update(&index.to_be_bytes());
    let result = mac.finalize().into_bytes();

    let (secret_key_tweak_bytes, chain_code_bytes) = result.split_at(SECRET_KEY_SIZE);

    let mut secret_key_tweak = [0u8; SECRET_KEY_SIZE];
    let mut new_chain_code = [0u8; 32];

    secret_key_tweak.copy_from_slice(secret_key_tweak_bytes);
    new_chain_code.copy_from_slice(chain_code_bytes);

    // Add tweak to parent secret key
    let tweak = Scalar::from_be_bytes(secret_key_tweak)
        //.map_err(|_| "Invalid tweak value")?;
        .map_err(|_| KeyDerivationError::InvalidTweakValue)?;
    let parent_sk = SecretKey::from_slice(&parent.secret_key)
        //.map_err(|_| "Invalid parent secret key")?;
        .map_err(|_| KeyDerivationError::InvalidSecretKey)?;

    let child_sk = parent_sk.add_tweak(&tweak)
        //.map_err(|_| "Invalid resulting secret key")?;
        .map_err(|_| KeyDerivationError::InvalidResultingSecretKey)?;

    Ok(ExtendedSecretKey::new(child_sk.secret_bytes(), new_chain_code))
}

/// Derives a child public key from an extended public key.
/// 
/// # Arguments
/// - `parent_pubkey`: The parent public key.
/// - `chain_code`: The chain code associated with the parent.
/// - `index`: The index of the child key.
fn derive_child_public_key(
    parent_pubkey: &PublicKey,
    chain_code: &[u8],
    index: u32,
) -> Result<PublicKey, KeyDerivationError> {
    if index >= HARDENED_OFFSET {
        //return Err("Cannot derive hardened key from public key");
        return Err(KeyDerivationError::PublicKeyHardenedDerivationError);
    }

    let mut mac = Hmac::<Sha512>::new_from_slice(chain_code)
        //.map_err(|_| "HMAC initialization failed")?;
        .map_err(|_| KeyDerivationError::HmacInitFailed)?;
    mac.update(&parent_pubkey.serialize());
    mac.update(&index.to_be_bytes());
    let result = mac.finalize().into_bytes();

    let (key_tweak_bytes, _new_chain_code) = result.split_at(SECRET_KEY_SIZE);

    let key_tweak_array: [u8; 32] = key_tweak_bytes
        .try_into()
        //.map_err(|_| "Invalid tweak size")?;
        .map_err(|_| KeyDerivationError::InvalidTweakSize)?;
    let tweak = Scalar::from_be_bytes(key_tweak_array)
        //.map_err(|_| "Invalid tweak value")?;
        .map_err(|_| KeyDerivationError::InvalidTweakValue)?;
    let secp = Secp256k1::new();
    let child_pubkey = parent_pubkey.add_exp_tweak(&secp, &tweak)
        //.map_err(|_| "Invalid resulting public key")?;
        .map_err(|_| KeyDerivationError::InvalidResultingPublicKey)?;

    Ok(child_pubkey)
}
