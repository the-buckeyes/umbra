use chrono::naive::NaiveDateTime;
use diesel::{ExpressionMethods, MysqlConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::algorithm;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Algorithm {
    pub id: u32,
    pub slug: String,
    pub display: String,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
    pub deleted: i64,
}

pub type AlgorithmColumns = (
    algorithm::id,
    algorithm::slug,
    algorithm::display,
    algorithm::created,
    algorithm::updated,
);

pub const ALGORITHM_COLUMNS: AlgorithmColumns = (
    algorithm::id,
    algorithm::slug,
    algorithm::display,
    algorithm::created,
    algorithm::updated,
);

impl Algorithm {
    pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
        use crate::schema::algorithm::dsl::*;

        Ok(algorithm.load::<Algorithm>(db)?)
    }

    pub fn get_by_slug(db: &MysqlConnection, algo_slug: &str) -> Result<Self, UmbraModelError> {
        use crate::schema::algorithm::dsl::*;

        Ok(algorithm.filter(slug.eq(algo_slug)).first(db)?)
    }
}
