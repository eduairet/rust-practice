use cryptography::{sha256_digest, sign_and_verify_hmac};
use data_encoding::HEXUPPER;
use ring::hmac;
use std::{
    fs::{remove_file, File},
    io::{BufReader, Write},
    path::Path,
};

#[cfg(test)]
mod tests_hashing {

    use super::*;

    #[test]
    #[ignore]
    fn test_sha256_digest() {
        let file = "file.txt";

        let mut output = File::create(file).unwrap();
        write!(output, "Hello, world!").unwrap();

        let input = File::open(file).unwrap();
        let reader = BufReader::new(input);
        let digest = sha256_digest(reader).unwrap();

        assert_eq!(
            HEXUPPER.encode(digest.as_ref()),
            "315F5BDB76D078C43B8AC0064E4A0164612B1FCE77C869345BFC94C75894EDD3"
        );

        remove_file(file).unwrap();
        assert!(Path::new(file).exists() == false);
    }

    #[test]
    fn test_sign_and_verify_hmac() {
        let message = "Hello, world!";
        let (key, signature) = sign_and_verify_hmac(message).unwrap();
        let verification = hmac::verify(&key, message.as_bytes(), signature.as_ref()).unwrap();
        assert_eq!(verification, ());
    }
}
