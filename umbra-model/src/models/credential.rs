use chrono::naive::NaiveDateTime;
use diesel::prelude::*;
use diesel::{ExpressionMethods, MysqlConnection, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::credential;

#[derive(Deserialize)]
pub struct Registration {
  pub system: String,
  pub organization: String,
  pub foreign_id: String,
  pub password: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Credential {
  pub id: u64,
  pub foreign_id: String,
  pub algorithm_id: u32,
  pub salt: Option<String>,
  pub derived_key: Option<String>,
  pub created: NaiveDateTime,
  pub updated: NaiveDateTime,
  pub deleted: i64,
}

pub type CredentialColumns = (
  credential::id,
  credential::foreign_id,
  credential::algorithm_id,
  credential::created,
  credential::updated,
);

pub const ALGORITHM_COLUMNS: CredentialColumns = (
  credential::id,
  credential::foreign_id,
  credential::algorithm_id,
  credential::created,
  credential::updated,
);

fn hash_foreign_id(
  system_id: &str,
  organization_id: &str,
  foreign_id: &str,
) -> String {
  use sha2::{Digest, Sha256};

  let mut hasher = Sha256::new();
  hasher.input(system_id);
  hasher.input(organization_id);
  hasher.input(foreign_id);

  format!("{:x}", hasher.result())
}

fn hash_password(
  registration: &Registration,
) -> Result<String, UmbraModelError> {
  use scrypt::{scrypt_simple, ScryptParams};

  let scrypt_params = match ScryptParams::new(15, 8, 1) {
    Ok(p) => p,
    Err(e) => return Err(UmbraModelError::CryptoError(format!("{}", e))),
  };

  let hash = match scrypt_simple(&registration.password, &scrypt_params) {
    Ok(hash) => hash,
    Err(e) => return Err(UmbraModelError::CryptoError(format!("{}", e))),
  };

  Ok(hash)
}

impl Credential {
  pub fn new(
    db: &MysqlConnection,
    registration: Registration,
  ) -> Result<Self, UmbraModelError> {
    use crate::models::{
      algorithm::Algorithm, organization::Organization, system::System,
    };
    use crate::schema::credential::dsl::*;
    use diesel::insert_into;

    let key = hash_password(&registration)?;

    let algorithm = Algorithm::get_by_slug(db, "scrypt")?;
    let organization =
      Organization::get_by_slug(db, &registration.organization)?;
    let system = System::get_by_slug(db, &registration.system)?;

    let user = db.transaction(|| {
      insert_into(credential)
        .values((
          algorithm_id.eq(algorithm.id),
          foreign_id.eq(hash_foreign_id(
            &system.id.to_string(),
            &organization.id.to_string(),
            &registration.foreign_id,
          )),
          derived_key.eq(key),
        ))
        .execute(db)?;

      credential.order(id.desc()).first(db)
    })?;

    Ok(user)
  }

  pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
    use crate::schema::credential::dsl::*;
    Ok(credential.load::<Credential>(db)?)
  }
}
