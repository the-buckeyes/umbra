use chrono::naive::NaiveDateTime;
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::system;

#[derive(Queryable, Serialize, Deserialize)]
pub struct System {
    pub id: u32,
    pub slug: String,
    pub display: String,
    pub cipher_key: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub deleted: i64,
}

pub type SystemColumns = (
    system::id,
    system::slug,
    system::display,
    system::cipher_key,
    system::created,
    system::updated,
);

pub const SYSTEM_COLUMNS: SystemColumns = (
    system::id,
    system::slug,
    system::display,
    system::cipher_key,
    system::created,
    system::updated,
);

impl System {
    pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
        use crate::schema::system::dsl::*;

        Ok(system.load::<System>(db)?)
    }

    pub fn get_by_slug(db: &MysqlConnection, s: &str) -> Result<Self, UmbraModelError> {
        use crate::schema::system::dsl::*;

        Ok(system.filter(slug.eq(s)).first(db)?)
    }
}
