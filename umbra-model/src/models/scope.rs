use chrono::naive::NaiveDateTime;
use diesel::{MysqlConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::scope;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Scope {
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

pub type ScopeColumns = (
    scope::id,
    scope::system_id,
    scope::organization_id,
    scope::slug,
    scope::display,
    scope::description,
    scope::created,
    scope::updated,
);

pub const SCOPE_COLUMNS: ScopeColumns = (
    scope::id,
    scope::system_id,
    scope::organization_id,
    scope::slug,
    scope::display,
    scope::description,
    scope::created,
    scope::updated,
);

impl Scope {
    pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
        use crate::schema::scope::dsl::*;

        Ok(scope.load::<Scope>(db)?)
    }
}
