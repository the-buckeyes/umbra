#[derive(Debug)]
pub enum Constraint {
    Conflict(String),
    Null(String),
}

#[derive(Debug)]
pub enum Error {
    Connection(String),
    Config(String),
    NotFound,
    Constraint(Constraint),
    Query(String),
    Decode(String),
}

impl From<mysql_async::error::Error> for Error {
    fn from(err: mysql_async::error::Error) -> Self {
        use mysql_async::error::Error as Fail;

        match err {
            Fail::Driver(e) => Error::Connection(format!("Internal driver failure: {:?}", e)),
            Fail::Io(e) => Error::Connection(format!("Input / Output stream failure: {:?}", e)),
            Fail::Other(e) => Error::Connection(format!("Unknown (other) failure: {:?}", e)),
            Fail::Server(e) => Error::Connection(format!("Server failure: {:?}", e)),
            Fail::Tls(e) => {
                Error::Connection(format!("Secure communication (TLS) failure: {:?}", e))
            }
            // @TODO - perhaps this shoulc be a configuration error?
            Fail::Url(e) => Error::Config(format!("Connection URL is not valid: {:?}", e)),
        }
    }
}

/*
impl From<&quaint::error::ErrorKind> for Error {
    fn from(e: &quaint::error::ErrorKind) -> Self {
        use quaint::error::ErrorKind as Fail;

        match e {
            Fail::NotFound => Error::NotFound,
            Fail::DatabaseDoesNotExist { db_name } => {
                Error::Config(format!("Database does not exist: {}", db_name))
            }
            Fail::DatabaseAlreadyExists { db_name } => Error::Constraint(Constraint::Conflict(
                format!("Database Name Conflict :: {}", db_name),
            )),
            Fail::DatabaseAccessDenied { db_name } => {
                Error::Config(format!("Database access denied: {}", db_name))
            }
            Fail::AuthenticationFailed { user } => {
                Error::Config(format!("Access Denied: {}", user))
            }
            Fail::DatabaseUrlIsInvalid(url) => {
                Error::Config(format!("Invalid database URL: {}", url))
            }
            Fail::InvalidConnectionArguments => {
                Error::Config("Database connection arguments were not correct".to_string())
            }
            Fail::ConnectionError(e) => {
                Error::Connection(format!("Unknown connection failure: {:?}", e))
            }
            Fail::IoError(e) => Error::Connection(format!("IO Error :: {:?}", e)),
            Fail::ConnectTimeout(message) | Fail::Timeout(message) => {
                Error::Connection(format!("Timout :: {}", message))
            }
            Fail::TlsError { message } => Error::Connection(format!("TLS Error :: {:?}", message)),
            Fail::UniqueConstraintViolation { constraint } => {
                Error::Constraint(Constraint::Conflict(format!("{:?}", constraint)))
            }
            Fail::NullConstraintViolation { constraint } => {
                Error::Constraint(Constraint::Null(format!("{:?}", constraint)))
            }
            Fail::QueryError(e) => Error::Query(format!("{:?}", e)),
            Fail::ColumnNotFound(name) => Error::Decode(format!("Column Not Found :: {}", name)),
            Fail::ColumnReadFailure(e) => {
                Error::Decode(format!("Column could not be read :: {:?}", e))
            }
            Fail::ResultTypeMismatch(reason) | Fail::ConversionError(reason) => {
                Error::Decode(format!("Conversion Failure :: {}", reason))
            }
            Fail::ResultIndexOutOfBounds(index) => {
                Error::Decode(format!("Index out of bounds :: {}", index))
            }
        }
    }
}

impl From<quaint::error::Error> for Error {
    fn from(e: quaint::error::Error) -> Self {
        e.kind().into()
    }
}
*/
