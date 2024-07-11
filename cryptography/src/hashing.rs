use ring::{
    digest::{Context, Digest, SHA256},
    error::Unspecified,
    hmac::{self, Key, Tag},
    rand::{self, SecureRandom},
};
use std::io::{Error, Read};

/// Computes the SHA-256 digest of the data read from the given reader.
///
/// # Arguments
///
/// * `reader` - The reader to read the data from.
///
/// # Returns
///
/// The SHA-256 digest of the data read from the given reader.
///
/// # Examples
///
/// ```ignore
/// use cryptography::sha256_digest;
/// use data_encoding::HEXUPPER;
/// use std::{
///    fs::{remove_file, File},
///    io::{BufReader, Write},
/// };
///
/// let file = "file.txt";
///
/// let mut output = File::create(file).unwrap();
/// write!(output, "Hello, world!").unwrap();
///
/// let input = File::open(file).unwrap();
/// let reader = BufReader::new(input);
/// let digest = sha256_digest(reader).unwrap();
///
/// println!("{}", HEXUPPER.encode(digest.as_ref()));
/// remove_file(file).unwrap();
/// ```
pub fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

/// Signs the HMAC of the given message.
///
/// # Arguments
///
/// * `message` - The message to sign.
///
/// # Returns
///
/// A tuple containing the key and the signature.
///
/// # Examples
///
/// ```
/// use cryptography::sign_and_verify_hmac;
/// use ring::hmac;
///
/// let message = "Hello, world!";
/// let (key, signature) = sign_and_verify_hmac(message).unwrap();
/// hmac::verify(&key, message.as_bytes(), signature.as_ref()).unwrap();
/// ```
pub fn sign_and_verify_hmac(message: &str) -> Result<(Key, Tag), Unspecified> {
    let mut key_value = [0u8; 32];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);
    let signature = hmac::sign(&key, message.as_bytes());
    Ok((key, signature))
}
