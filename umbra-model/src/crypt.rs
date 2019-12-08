pub mod hash {
  use crate::errors::UmbraModelError;

  pub fn foreign_id(
    system_key: &str,
    organization_key: &str,
    foreign_id: &str,
  ) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.input(system_key);
    hasher.input(organization_key);
    hasher.input(foreign_id);

    format!("{:x}", hasher.result())
  }

  pub fn password(plaintext: &str) -> Result<String, UmbraModelError> {
    scrypt::ScryptParams::new(15, 8, 1)
      .map_err(|e| UmbraModelError::CryptoError(format!("{}", e)))
      .and_then(|params| {
        scrypt::scrypt_simple(plaintext, &params)
          .map_err(|e| UmbraModelError::CryptoError(format!("{}", e)))
      })
  }
}
