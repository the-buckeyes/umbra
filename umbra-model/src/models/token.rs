use chrono::naive::NaiveDateTime;
use diesel::{MysqlConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::token;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Token {
  pub id: u64,
  pub token_kind_id: u32,
  pub proof: String,
  pub usage_count: u32,
  pub expiration: i64,
  pub created: NaiveDateTime,
  pub updated: NaiveDateTime,
  pub deleted: i64,
}

pub type TokenColumns = (
  token::id,
  token::token_kind_id,
  token::proof,
  token::usage_count,
  token::expiration,
  token::created,
  token::updated,
);

pub const TOKEN_COLUMNS: TokenColumns = (
  token::id,
  token::token_kind_id,
  token::proof,
  token::usage_count,
  token::expiration,
  token::created,
  token::updated,
);

impl Token {
  pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
    use crate::schema::token::dsl::*;

    Ok(token.load::<Token>(db)?)
  }
}
