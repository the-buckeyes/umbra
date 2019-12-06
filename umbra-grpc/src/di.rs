use umbra_model::{
  db::{MyPool, MyPooledConnection},
  errors::UmbraModelError,
};

pub type DBPool = MyPool;

pub type DBConn = MyPooledConnection;

pub type DBContainer = std::sync::Arc<DBPool>;

pub fn db_acquire(container: DBContainer) -> Result<DBConn, UmbraModelError> {
  umbra_model::db::acquire(container)
}
