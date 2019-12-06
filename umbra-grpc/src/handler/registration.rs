use crate::errors::UmbraGrpcError;
use crate::umbra_auth::{Identity, IdentityReply, Meta, Registration};
use umbra_model::{
  db,
  models::credential::{
    Credential as IdentityModel, Registration as RegistrationModel,
  },
};

pub type Request = tonic::Request<Registration>;

pub type Reply = Result<tonic::Response<IdentityReply>, tonic::Status>;

pub fn register(
  db: crate::di::DBContainer,
  r: &Registration,
) -> Result<Identity, UmbraGrpcError> {
  let registration = RegistrationModel {
    organization: r.organization_slug.clone(),
    system: r.system_slug.clone(),
    foreign_id: r.username.clone(),
    password: r.password.clone(),
  };

  let db = db::acquire(db)?;
  let identity = IdentityModel::new(&db, registration)?;

  Ok(Identity {
    meta: Some(Meta {
      id: identity.id.to_string(),
      created: identity.created.to_string(),
      updated: identity.updated.to_string(),
      deleted: identity.deleted.to_string(),
    }),
    foreign_id: identity.foreign_id,
  })
}
