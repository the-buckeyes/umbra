use super::{
  algorithm::Algorithm, identity::Identity, organization::Organization,
  reply::Reply, system::System,
};
use crate::errors::UmbraModelError;
use mysql_async::Conn as MySql;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Registration<'a> {
  pub system: &'a str,
  pub organization: &'a str,
  pub username: &'a str,
  pub password: &'a str,
}

fn invalid(field_list: &str) -> Result<String, UmbraModelError> {
  let message = format!("NOT FOUND :: {}", field_list);
  let error = UmbraModelError::ValidationFailure(message);

  Err(error)
}

fn get_username_hash(
  username: &str,
  algorithm: Option<Algorithm>,
  organization: Option<Organization>,
  system: Option<System>,
) -> Result<String, UmbraModelError> {
  let (_, system_key, organizion_key) = match (algorithm, system, organization)
  {
    (Some(a), Some(s), Some(o)) => (a.id, s.cipher_key, o.cipher_key),
    (None, None, None) => return invalid("algorithm, system, organization"),
    (None, None, Some(_)) => return invalid("algorithm, system"),
    (None, Some(_), None) => return invalid("algorithm, organization"),
    (Some(_), None, None) => return invalid("system, organization"),
    (Some(_), Some(_), None) => return invalid("organization"),
    (Some(_), None, Some(_)) => return invalid("system"),
    (None, Some(_), Some(_)) => return invalid("algorithm"),
  };

  Ok(crate::crypt::hash::username(
    &system_key,
    &organizion_key,
    username,
  ))
}

impl<'a> Registration<'a> {
  pub async fn into_identity(
    db: MySql,
    r: &Registration<'a>,
  ) -> Reply<Option<Identity>> {
    let (db, algo) = Algorithm::get_by_slug(db, "BCRYPT")
      .await
      .map_err(|e| UmbraModelError::from(e))
      .and_then(|(db, option)| {
        let message = String::from("BAD ALGORITHM");
        option
          .ok_or_else(|| UmbraModelError::CryptoError(message))
          .map(|algo| (db, algo))
      })?;
    let (db, org) = match Organization::get_by_slug(db, r.organization).await {
      Ok(tuple) => tuple,
      Err(error) => return Err(error),
    };
    let (db, sys) = match System::get_by_slug(db, r.system).await {
      Ok(tuple) => tuple,
      Err(error) => return Err(error),
    };

    let derived_key = crate::crypt::hash::password(r.password)?;
    let algorithm_id = algo.id.clone();

    let username_hash =
      get_username_hash(r.username, Some(algo), org, sys)?;

    Identity::insert(db, &username_hash, algorithm_id, None, &derived_key)
      .await
  }
}
