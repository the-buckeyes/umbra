use mysql_async::{Conn as MySql, prelude::{params, Queryable}};

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
}
