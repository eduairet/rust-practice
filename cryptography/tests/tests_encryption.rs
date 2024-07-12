use cryptography::salt_and_hash_password_with_pbkdf2;
use ring::pbkdf2::{verify, PBKDF2_HMAC_SHA512};
use std::num::NonZeroU32;

#[cfg(test)]
mod tests_encryption {

    use super::*;

    #[test]
    fn test_salt_and_hash_password_with_pbkdf2() {
        let password = "ABCabc123?";
        let (salt, mut hash) = salt_and_hash_password_with_pbkdf2(password).unwrap();

        let iterations = NonZeroU32::new(100_000).unwrap();

        let should_succeed = verify(
            PBKDF2_HMAC_SHA512,
            iterations,
            &salt,
            password.as_bytes(),
            &mut hash,
        );

        assert!(should_succeed.is_ok());

        let wrong_password = "DEFdef456&";
        let should_fail = verify(
            PBKDF2_HMAC_SHA512,
            iterations,
            &salt,
            wrong_password.as_bytes(),
            &hash,
        );

        assert!(!should_fail.is_ok());
    }
}
