use chrono::naive::NaiveDateTime;
use diesel::{MysqlConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::role_scope;

#[derive(Queryable, Serialize, Deserialize)]
pub struct RoleScope {
  pub id: u64,
  pub role_id: u64,
  pub scope_id: u64,
  pub created: NaiveDateTime,
  pub updated: NaiveDateTime,
  pub deleted: i64,
}

pub type RoleScopeColumns = (
  role_scope::id,
  role_scope::role_id,
  role_scope::scope_id,
  role_scope::created,
  role_scope::updated,
);

pub const ROLE_SCOPE_COLUMNS: RoleScopeColumns = (
  role_scope::id,
  role_scope::role_id,
  role_scope::scope_id,
  role_scope::created,
  role_scope::updated,
);

impl RoleScope {
  pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
    use crate::schema::role_scope::dsl::*;

    Ok(role_scope.load::<RoleScope>(db)?)
  }
}
