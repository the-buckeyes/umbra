use diesel::{
  self,
  r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use dotenv::dotenv;
use std::env;

use crate::errors::UmbraModelError;

pub type MyPool = Pool<ConnectionManager<diesel::MysqlConnection>>;
pub type MyPooledConnection =
  PooledConnection<ConnectionManager<diesel::MysqlConnection>>;

pub fn init_pool(database_url: &str) -> Result<MyPool, PoolError> {
  let manager = ConnectionManager::<diesel::MysqlConnection>::new(database_url);
  Pool::builder().build(manager)
}

pub fn connect() -> MyPool {
  dotenv().ok();

  let database_url =
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  init_pool(&database_url).expect("Failed to create MySQL connection pool")
}

pub fn acquire(
  db: std::sync::Arc<MyPool>,
) -> Result<MyPooledConnection, UmbraModelError> {
  Ok(db.clone().get()?)
}
