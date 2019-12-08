use chrono::naive::NaiveDateTime;
use diesel::{MysqlConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::role;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Role {
    pub id: u64,
    pub system_id: u32,
    pub organization_id: u32,
    pub slug: String,
    pub display: String,
    pub description: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub deleted: i64,
}

pub type RoleColumns = (
    role::id,
    role::system_id,
    role::organization_id,
    role::slug,
    role::display,
    role::description,
    role::created,
    role::updated,
);

pub const ALGORITHM_COLUMNS: RoleColumns = (
    role::id,
    role::system_id,
    role::organization_id,
    role::slug,
    role::display,
    role::description,
    role::created,
    role::updated,
);

impl Role {
    pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
        use crate::schema::role::dsl::*;

        Ok(role.load::<Role>(db)?)
    }
}
