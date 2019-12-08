use mysql_async::{
  prelude::{params, Queryable},
  Conn as MySql,
};

use crate::errors::UmbraModelError;

use super::reply::Reply;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Credential {
  pub id: u64,
  pub foreign_id: String,
  pub algorithm_id: u32,
  pub salt: Option<String>,
  pub derived_key: Option<String>,
  pub created: chrono::naive::NaiveDateTime,
  pub updated: chrono::naive::NaiveDateTime,
  pub deleted: i64,
}

type Row = (
  u64,
  String,
  u32,
  Option<String>,
  Option<String>,
  chrono::naive::NaiveDateTime,
  chrono::naive::NaiveDateTime,
  i64,
);

impl From<Row> for Credential {
  fn from(row: Row) -> Self {
    Credential {
      id: row.0,
      foreign_id: row.1,
      algorithm_id: row.2,
      salt: row.3,
      derived_key: row.4,
      created: row.5,
      updated: row.6,
      deleted: row.7,
    }
  }
}

const BASE_QUERY: &'static str = "
SELECT
  id
, foreign_id
, algorithm_id
, salt
, derived_key
, created
, updated
, deleted
FROM credential
";

fn validation_failure(field: &str) -> UmbraModelError {
  let message = format!("NOT FOUND :: {}", field);
  UmbraModelError::ValidationFailure(message)
}

impl Credential {
  pub async fn get_by_id(db: MySql, id: u64) -> Reply<Option<Self>> {
    let sql = format!("{} WHERE id = :id", BASE_QUERY);
    let params = params! {
        "id" => id
    };
    let (db, row): (_, Option<Row>) = db.first_exec(sql, params).await?;
    let reply = row.map(|r| Credential::from(r));

    Ok((db, reply))
  }

  pub async fn get_by_foreign_id_hash(
    db: MySql,
    hash: &str,
  ) -> Reply<Option<Self>> {
    let sql = format!("{} WHERE foreign_id = :foreign_id", BASE_QUERY);
    let params = params! {
        "foreign_id" => hash
    };
    let (db, row): (_, Option<Row>) = db.first_exec(sql, params).await?;
    let reply = row.map(|r| Credential::from(r));

    Ok((db, reply))
  }

  pub async fn get_by_foreign_id(
    db: MySql,
    system: &str,
    organization: &str,
    foreign_id: &str,
  ) -> Reply<Option<Self>> {
    use crate::crypt;
    use crate::mysql::{organization::Organization, system::System};

    let sql = format!("{} WHERE foreign_id = :foreign_id", BASE_QUERY);

    let (db, org) = Organization::get_by_slug(db, organization)
      .await
      .map_err(|e| UmbraModelError::from(e))
      .and_then(|(db, option)| {
        option
          .ok_or_else(|| validation_failure("organization"))
          .map(|org| (db, org))
      })?;
    let (db, sys) = System::get_by_slug(db, system)
      .await
      .map_err(|e| UmbraModelError::from(e))
      .and_then(|(db, option)| {
        option
          .ok_or_else(|| validation_failure("system"))
          .map(|sys| (db, sys))
      })?;

    let foreign_id = crypt::hash::foreign_id(
      &sys.id.to_string(),
      &org.id.to_string(),
      foreign_id,
    );
    let params = params! {
        "foreign_id" => foreign_id,
    };
    let (db, row): (_, Option<Row>) = db.first_exec(sql, params).await?;
    let reply = row.map(|r| Credential::from(r));

    Ok((db, reply))
  }

  pub async fn insert(
    db: MySql,
    foreign_id: &str,
    algorithm_id: u32,
    salt: Option<&str>,
    derived_key: &str,
  ) -> Reply<Option<Self>> {
    let sql = "
          INSERT INTO `credential`
          (foreign_id, algorithm_id, salt, derived_key)
          VALUES
          (:foreign_id, :algorithm_id, :salt, :derived_key)
        ";
    let params = params! {
        "foreign_id" => foreign_id,
        "algorithm_id" => algorithm_id,
        "salt" => salt,
        "derived_key" => Some(derived_key),
    };

    let db = db
      .drop_exec(sql, params)
      .await
      .map_err(|e| UmbraModelError::from(e))?;

    let (db, credential) =
      self::Credential::get_by_foreign_id_hash(db, foreign_id).await?;

    Ok((db, credential))
  }
}
