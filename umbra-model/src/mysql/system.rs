use mysql_async::{
  prelude::{params, Queryable},
  Conn as MySql,
};

use super::reply::Reply;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct System {
  pub id: u32,
  pub slug: String,
  pub display: String,
  pub cipher_key: String,
  pub created: chrono::naive::NaiveDateTime,
  pub updated: chrono::naive::NaiveDateTime,
  pub deleted: i64,
}

type Row = (
  u32,
  String,
  String,
  String,
  chrono::naive::NaiveDateTime,
  chrono::naive::NaiveDateTime,
  i64,
);

impl From<Row> for System {
  fn from(row: Row) -> Self {
    System {
      id: row.0,
      slug: row.1,
      display: row.2,
      cipher_key: row.3,
      created: row.4,
      updated: row.5,
      deleted: row.6,
    }
  }
}

const BASE_QUERY: &'static str = "
SELECT
  id
, slug
, display
, cipher_key
, created
, updated
, deleted
FROM `system`
";

impl System {
  pub async fn list(db: MySql) -> Reply<Vec<Self>> {
    let result = db.prep_exec(BASE_QUERY, ()).await?;
    let (db, list) = result
      .map_and_drop(|row| {
        let (id, slug, display, cipher_key, created, updated, deleted) =
          mysql_async::from_row(row);

        System {
          id,
          slug,
          display,
          cipher_key,
          created,
          updated,
          deleted,
        }
      })
      .await?;

    Ok((db, list))
  }

  pub async fn get_by_id(db: MySql, id: u32) -> Reply<Option<Self>> {
      let sql = format!("{} WHERE id = :id", BASE_QUERY);
      let params  = params! { "id" => id };
      let (db, row): (_, Option<Row>) = db.first_exec(sql, params).await?;
      let reply = row.map(|r| System::from(r));

      Ok((db, reply))
  }
}

