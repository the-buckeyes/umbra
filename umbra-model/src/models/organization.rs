use chrono::naive::NaiveDateTime;
use diesel::{
  ExpressionMethods, MysqlConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::errors::UmbraModelError;
use crate::schema::organization;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Organization {
  pub id: u32,
  pub slug: String,
  pub display: String,
  pub cipher_key: String,
  pub created: NaiveDateTime,
  pub updated: NaiveDateTime,
  pub deleted: i64,
}

pub type OrganizationColumns = (
  organization::id,
  organization::slug,
  organization::display,
  organization::cipher_key,
  organization::created,
  organization::updated,
);

pub const ORGANIZATION_COLUMNS: OrganizationColumns = (
  organization::id,
  organization::slug,
  organization::display,
  organization::cipher_key,
  organization::created,
  organization::updated,
);

impl Organization {
  pub fn list(db: &MysqlConnection) -> Result<Vec<Self>, UmbraModelError> {
    use crate::schema::organization::dsl::*;

    Ok(organization.load::<Organization>(db)?)
  }

  pub fn get_by_slug(
    db: &MysqlConnection,
    org_slug: &str,
  ) -> Result<Self, UmbraModelError> {
    use crate::schema::organization::dsl::*;

    Ok(organization.filter(slug.eq(org_slug)).first(db)?)
  }
}
