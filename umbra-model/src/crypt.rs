pub mod hash {
    use crate::errors::UmbraModelError;
    use std::time::Instant;

    pub fn username(system_key: &str, organization_key: &str, username_hash: &str) -> String {
        let now = Instant::now();

        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.input(system_key);
        hasher.input(organization_key);
        hasher.input(username_hash);

        println!("crypt::hash::username - {}us", now.elapsed().as_micros());

        format!("{:x}", hasher.result())
    }

    pub fn password(plaintext: &str) -> Result<String, UmbraModelError> {
        let now = Instant::now();

        let hash = scrypt::ScryptParams::new(15, 8, 1)
            .map_err(|e| UmbraModelError::CryptoError(format!("{}", e)))
            .and_then(|params| {
                scrypt::scrypt_simple(plaintext, &params)
                    .map_err(|e| UmbraModelError::CryptoError(format!("{}", e)))
            })?;

        println!("crypt::hash::password - {}us", now.elapsed().as_micros());

        Ok(hash)
    }
}
