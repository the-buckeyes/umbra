use dotenv::dotenv;
use mysql_async::{Conn as MySql, Pool};
use std::env;

use crate::errors::UmbraModelError;

pub type MySqlPool = Pool;

pub type MySqlConnection = MySql;

pub fn init_pool(database_url: &str) -> Pool {
    Pool::new(database_url)
}

pub fn connect() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    init_pool(&database_url)
}

pub async fn acquire(db: std::sync::Arc<Pool>) -> Result<MySql, UmbraModelError> {
    Ok(db.clone().get_conn().await?)
}
