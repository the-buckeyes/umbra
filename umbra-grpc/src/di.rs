pub use umbra_model::mysql::connection::{MySqlConnection, MySqlPool};
use umbra_model::{
  db::{MyPool, MyPooledConnection},
  errors::UmbraModelError,
};

pub type DBPool = MyPool;

pub type DBConn = MyPooledConnection;

pub type DBContainer = std::sync::Arc<DBPool>;

pub type MySqlContainer = std::sync::Arc<MySqlPool>;

pub fn db_acquire(container: DBContainer) -> Result<DBConn, UmbraModelError> {
  umbra_model::db::acquire(container)
}

pub async fn mysql_acquire(
  container: MySqlContainer,
) -> Result<MySqlConnection, UmbraModelError> {
  umbra_model::mysql::connection::acquire(container).await
}
