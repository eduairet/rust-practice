use ring::{
    digest::SHA1_OUTPUT_LEN,
    pbkdf2::{derive, PBKDF2_HMAC_SHA512},
    rand::{SecureRandom, SystemRandom},
};
use std::{error::Error, num::NonZeroU32};

const CREDENTIAL_LEN: usize = SHA1_OUTPUT_LEN;

/// Salts and hashes a password using PBKDF2 with HMAC-SHA512.
///
/// # Arguments
///
/// * `password` - The password to salt and hash.
///
/// # Returns
///
/// A tuple containing the salt and the hashed password.
///
/// # Example
///
/// ```
/// use cryptography::salt_and_hash_password_with_pbkdf2;
/// use data_encoding::HEXUPPER;
///
/// let password = "ABCabc123?";
/// let (salt, hash) = salt_and_hash_password_with_pbkdf2(password).unwrap();
///
/// println!("Salt: {}", HEXUPPER.encode(&salt));
/// println!("Hash: {}", HEXUPPER.encode(&hash));
/// ```
pub fn salt_and_hash_password_with_pbkdf2(
    password: &str,
) -> Result<([u8; 20], [u8; 20]), Box<dyn Error>> {
    let iterations = NonZeroU32::new(100_000).unwrap();
    let range = SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    range
        .fill(&mut salt)
        .map_err(|_| Box::<dyn Error>::from("Failed to generate random salt"))?;

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    derive(
        PBKDF2_HMAC_SHA512,
        iterations,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    Ok((salt, pbkdf2_hash))
}
