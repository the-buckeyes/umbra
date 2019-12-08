use diesel;
use std::fmt;

#[derive(Debug)]
pub enum UmbraModelError {
    CryptoError(String),
    Failure(String),
    ValidationFailure(String),
    NotFound,
    Rollback,
}

impl From<diesel::result::Error> for UmbraModelError {
    fn from(error: diesel::result::Error) -> Self {
        use diesel::result::Error as DBFail;

        match error {
            DBFail::NotFound => Self::NotFound,
            DBFail::AlreadyInTransaction => Self::Failure(String::from(
                "Transaction Failure :: cannot begin while already in a transaction",
            )),
            DBFail::RollbackTransaction => Self::Rollback,
            DBFail::SerializationError(e) => {
                Self::Failure(format!("Serialization Failure :: {}", e))
            }
            DBFail::DeserializationError(e) => {
                Self::Failure(format!("Deserialization Failure :: {}", e))
            }
            DBFail::QueryBuilderError(e) => Self::Failure(format!("{}", e)),
            DBFail::DatabaseError(kind, error) => {
                Self::Failure(format!("{:?} :: {:?}", kind, error))
            }
            DBFail::InvalidCString(e) => panic!("Diesel Library Error :: {}", e),
            error => Self::Failure(format!("{:?}", error)),
        }
    }
}

impl From<mysql_async::error::Error> for UmbraModelError {
    fn from(error: mysql_async::error::Error) -> Self {
        use mysql_async::error::Error as DBFail;

        match error {
            DBFail::Driver(e) => Self::Failure(format!("{}", e)),
            DBFail::Io(e) => Self::Failure(format!("{}", e)),
            DBFail::Other(str) => Self::Failure(format!("{}", str)),
            DBFail::Server(e) => Self::Failure(format!("{}", e)),
            DBFail::Tls(e) => Self::Failure(format!("{}", e)),
            DBFail::Url(e) => Self::Failure(format!("{}", e)),
        }
    }
}

impl From<r2d2::Error> for UmbraModelError {
    fn from(error: r2d2::Error) -> Self {
        UmbraModelError::Failure(format!("{}", error))
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
            Self::CryptoError(reason) => write!(f, "encryption failure: {}", reason),
            Self::Failure(error) => write!(f, "{}", error),
            Self::ValidationFailure(message) => write!(f, "{}", message),
            Self::NotFound => write!(f, "Database record not found"),
            Self::Rollback => write!(f, "Database transaction was rolled back"),
        }
    }
}
