use mysql_async::{
  prelude::{params, Queryable},
  Conn as MySql,
};

use super::reply::Reply;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Algorithm {
  pub id: u32,
  pub slug: String,
  pub display: String,
  pub created: chrono::naive::NaiveDateTime,
  pub updated: chrono::naive::NaiveDateTime,
  pub deleted: i64,
}

type Row = (
  u32,
  String,
  String,
  chrono::naive::NaiveDateTime,
  chrono::naive::NaiveDateTime,
  i64,
);

impl From<Row> for Algorithm {
  fn from(row: Row) -> Self {
    Algorithm {
      id: row.0,
      slug: row.1,
      display: row.2,
      created: row.3,
      updated: row.4,
      deleted: row.5,
    }
  }
}

const BASE_QUERY: &'static str = "
SELECT
  id
, slug
, display
, created
, updated
, deleted
FROM algorithm
";

impl Algorithm {
  pub async fn list(db: MySql) -> Reply<Vec<Self>> {
    let result = db.prep_exec(BASE_QUERY, ()).await?;
    let (db, list) = result
      .map_and_drop(|row| {
        let (id, slug, display, created, updated, deleted) =
          mysql_async::from_row(row);

        Algorithm {
          id,
          slug,
          display,
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
      let reply = row.map(|r| Algorithm::from(r));

      Ok((db, reply))
  }
}
