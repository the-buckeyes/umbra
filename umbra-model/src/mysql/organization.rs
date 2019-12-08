use mysql_async::{
  prelude::{params, Queryable},
  Conn as MySql,
};

use super::reply::Reply;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Organization {
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

impl From<Row> for Organization {
  fn from(row: Row) -> Self {
    Organization {
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
FROM `organization`
";

impl Organization {
  pub async fn list(db: MySql) -> Reply<Vec<Self>> {
    let result = db.prep_exec(BASE_QUERY, ()).await?;
    let (db, list) = result
      .map_and_drop(|row| {
        let (id, slug, display, cipher_key, created, updated, deleted) =
          mysql_async::from_row(row);

        Organization {
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
    let params = params! { "id" => id };
    let (db, row): (_, Option<Row>) = db.first_exec(sql, params).await?;
    let reply = row.map(|r| Organization::from(r));

    Ok((db, reply))
  }

  pub async fn get_by_slug(db: MySql, slug: &str) -> Reply<Option<Self>> {
    let sql = format!("{} WHERE slug = :slug", BASE_QUERY);
    let params = params! { "slug" => slug };
    let (db, row): (_, Option<Row>) = db.first_exec(sql, params).await?;
    let reply = row.map(|r| Organization::from(r));

    Ok((db, reply))
  }

  pub async fn get_id_by_slug(db: MySql, slug: &str) -> Reply<Self> {
    self::Organization::get_by_slug(db, slug)
      .await
      .and_then(|(db, option)| {
        option
          .ok_or_else(|| {
            let message = String::from("NOT FOUND :: organization");
            crate::errors::UmbraModelError::ValidationFailure(message)
          })
          .map(|system| (db, system))
      })
  }
}
