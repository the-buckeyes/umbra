use chrono::naive::NaiveDateTime;
use diesel::{MysqlConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::token_data;

#[derive(Queryable, Serialize, Deserialize)]
pub struct TokenData {
    pub id: u64,
    pub token_id: u64,
    pub label: String,
    pub ciphertext: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub deleted: i64,
}

pub type TokenDataColumns = (
    token_data::id,
    token_data::token_id,
    token_data::label,
    token_data::ciphertext,
    token_data::created,
    token_data::updated,
);

pub const TOKEN_DATA_COLUMNS: TokenDataColumns = (
    token_data::id,
    token_data::token_id,
    token_data::label,
    token_data::ciphertext,
    token_data::created,
    token_data::updated,
);

impl TokenData {
    pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
        use crate::schema::token_data::dsl::*;

        Ok(token_data.load::<TokenData>(db)?)
    }
}
