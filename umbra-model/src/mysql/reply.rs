use mysql_async::Conn as MySql;

pub type Reply<T> = Result<(MySql, T), crate::errors::UmbraModelError>;
