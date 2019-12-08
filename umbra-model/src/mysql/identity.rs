use mysql_async::{
    prelude::{params, Queryable},
    Conn as MySql,
};

use crate::errors::UmbraModelError;

use super::reply::Reply;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Identity {
    pub id: u64,
    pub username_hash: String,
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

impl From<Row> for Identity {
    fn from(row: Row) -> Self {
        Identity {
            id: row.0,
            username_hash: row.1,
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
, username_hash
, algorithm_id
, salt
, derived_key
, created
, updated
, deleted
FROM identity
";

fn validation_failure(field: &str) -> UmbraModelError {
    let message = format!("NOT FOUND :: {}", field);
    UmbraModelError::ValidationFailure(message)
}

impl Identity {
    pub async fn get_by_id(db: MySql, id: u64) -> Reply<Option<Self>> {
        let sql = format!("{} WHERE id = :id", BASE_QUERY);
        let params = params! {
            "id" => id
        };
        let (db, row): (_, Option<Row>) = db.first_exec(sql, params).await?;
        let reply = row.map(|r| Identity::from(r));

        Ok((db, reply))
    }

    pub async fn get_by_username_hash(db: MySql, hash: &str) -> Reply<Option<Self>> {
        let sql = format!("{} WHERE username_hash = :username_hash", BASE_QUERY);
        let params = params! {
            "username_hash" => hash
        };
        let (db, row): (_, Option<Row>) = db.first_exec(sql, params).await?;
        let reply = row.map(|r| Identity::from(r));

        Ok((db, reply))
    }

    pub async fn get_by_username(
        db: MySql,
        system: &str,
        organization: &str,
        username: &str,
    ) -> Reply<Option<Self>> {
        use crate::crypt;
        use crate::mysql::{organization::Organization, system::System};

        let sql = format!("{} WHERE username_hash = :username_hash", BASE_QUERY);

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

        let username_hash =
            crypt::hash::username(&sys.id.to_string(), &org.id.to_string(), username);
        let params = params! {
            "username_hash" => username_hash,
        };
        let (db, row): (_, Option<Row>) = db.first_exec(sql, params).await?;
        let reply = row.map(|r| Identity::from(r));

        Ok((db, reply))
    }

    pub async fn insert(
        db: MySql,
        username_hash: &str,
        algorithm_id: u32,
        salt: Option<&str>,
        derived_key: &str,
    ) -> Reply<Option<Self>> {
        let sql = "
          INSERT INTO `identity`
          (username_hash, algorithm_id, salt, derived_key)
          VALUES
          (:username_hash, :algorithm_id, :salt, :derived_key)
        ";
        let params = params! {
            "username_hash" => username_hash,
            "algorithm_id" => algorithm_id,
            "salt" => salt,
            "derived_key" => Some(derived_key),
        };

        let db = db
            .drop_exec(sql, params)
            .await
            .map_err(|e| UmbraModelError::from(e))?;

        let (db, identity) = self::Identity::get_by_username_hash(db, username_hash).await?;

        Ok((db, identity))
    }
}
