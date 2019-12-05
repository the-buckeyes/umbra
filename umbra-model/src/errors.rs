use diesel;
use std::fmt;

#[derive(Debug)]
pub enum UmbraModelError {
  CryptoError(String),
  DBError(diesel::result::Error),
  MysqlConnectionError(String),
}

impl From<diesel::result::Error> for UmbraModelError {
  fn from(error: diesel::result::Error) -> Self {
    UmbraModelError::DBError(error)
  }
}

impl From<r2d2::Error> for UmbraModelError {
  fn from(error: r2d2::Error) -> Self {
    UmbraModelError::MysqlConnectionError(format!("{}", error))
  }
}

impl From<scrypt::errors::InvalidParams> for UmbraModelError {
  fn from(error: scrypt::errors::InvalidParams) -> Self {
    UmbraModelError::CryptoError(format!("{}", error))
  }
}

impl From<scrypt::errors::InvalidOutputLen> for UmbraModelError {
  fn from(error: scrypt::errors::InvalidOutputLen) -> Self {
    UmbraModelError::CryptoError(format!("{}", error))
  }
}

impl fmt::Display for UmbraModelError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      UmbraModelError::CryptoError(reason) => {
        write!(f, "encryption failure: {}", reason)
      }
      UmbraModelError::DBError(error) => write!(f, "{}", error),
      UmbraModelError::MysqlConnectionError(reason) => {
        write!(f, "failed to obtain a database connection: {}", reason)
      }
    }
  }
}
