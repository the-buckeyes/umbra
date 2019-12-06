#[derive(serde::Serialize, serde::Deserialize)]
pub struct Registration<'a> {
  pub system: &'a str,
  pub organization: &'a str,
  pub foreign_id: &'a str,
  pub password: &'a str,
}
