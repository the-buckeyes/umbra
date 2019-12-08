use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use diesel::{ExpressionMethods, MysqlConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::identity;

#[derive(Deserialize)]
pub struct Registration {
  pub system: String,
  pub organization: String,
  pub username: String,
  pub password: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Identity {
  pub id: u64,
  pub username_hash: String,
  pub algorithm_id: u32,
  pub salt: Option<String>,
  pub derived_key: Option<String>,
  pub created: NaiveDateTime,
  pub updated: NaiveDateTime,
  pub deleted: i64,
}

pub type IdentityColumns = (
  identity::id,
  identity::username_hash,
  identity::algorithm_id,
  identity::created,
  identity::updated,
);

pub const ALGORITHM_COLUMNS: IdentityColumns = (
  identity::id,
  identity::username_hash,
  identity::algorithm_id,
  identity::created,
  identity::updated,
);

impl Identity {
  pub fn new(
    db: &MysqlConnection,
    registration: Registration,
  ) -> Result<Self, UmbraModelError> {
    use crate::models::{
      algorithm::Algorithm, organization::Organization, system::System,
    };
    use crate::schema::identity::dsl::*;
    use diesel::insert_into;

    let key = crate::crypt::hash::password(&registration.password)?;

    let algorithm = Algorithm::get_by_slug(db, "scrypt")?;
    let organization =
      Organization::get_by_slug(db, &registration.organization)?;
    let system = System::get_by_slug(db, &registration.system)?;

    let username_hash_key = crate::crypt::hash::username(
      &system.id.to_string(),
      &organization.id.to_string(),
      &registration.username,
    );

    let user = db.transaction(|| {
      insert_into(identity)
        .values((
          algorithm_id.eq(algorithm.id),
          username_hash.eq(username_hash_key),
          derived_key.eq(key),
        ))
        .execute(db)?;

      identity.order(id.desc()).first(db)
    })?;

    Ok(user)
  }

  pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
    use crate::schema::identity::dsl::*;
    Ok(identity.load::<Identity>(db)?)
  }
}
