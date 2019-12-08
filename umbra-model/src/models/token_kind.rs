use chrono::naive::NaiveDateTime;
use diesel::{MysqlConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::token_kind;

#[derive(Queryable, Serialize, Deserialize)]
pub struct TokenKind {
    pub id: u32,
    pub slug: String,
    pub display: String,
    pub valid_duration_seconds: i32,
    pub is_single_use: bool,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub deleted: i64,
}

pub type TokenKindColumns = (
    token_kind::id,
    token_kind::slug,
    token_kind::display,
    token_kind::valid_duration_seconds,
    token_kind::is_single_use,
    token_kind::created,
    token_kind::updated,
);

pub const TOKEN_KIND_COLUMNS: TokenKindColumns = (
    token_kind::id,
    token_kind::slug,
    token_kind::display,
    token_kind::valid_duration_seconds,
    token_kind::is_single_use,
    token_kind::created,
    token_kind::updated,
);

impl TokenKind {
    pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
        use crate::schema::token_kind::dsl::*;

        Ok(token_kind.load::<TokenKind>(db)?)
    }
}
