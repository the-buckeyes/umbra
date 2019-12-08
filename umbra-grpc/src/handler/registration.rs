use crate::umbra_auth::{Identity, IdentityReply, Meta, Registration};
use crate::{di, errors::UmbraGrpcError};
use umbra_model::mysql::registration::Registration as RegistrationModel;

pub type Request = tonic::Request<Registration>;

pub type Reply = Result<tonic::Response<IdentityReply>, tonic::Status>;

pub async fn register(
    mysql: di::MySqlContainer,
    r: &Registration,
) -> Result<Identity, UmbraGrpcError> {
    let db = di::mysql_acquire(mysql).await?;
    let registration = RegistrationModel {
        organization: &r.organization_slug,
        system: &r.system_slug,
        username: &r.username,
        password: &r.password,
    };
    let (_, identity) = RegistrationModel::into_identity(db, &registration).await?;
    let identity = identity.ok_or_else(|| {
        let message = String::from("Identity create response was empty");
        UmbraGrpcError::Internal(message)
    })?;

    Ok(Identity {
        meta: Some(Meta {
            id: identity.id.to_string(),
            created: identity.created.to_string(),
            updated: identity.updated.to_string(),
            deleted: identity.deleted.to_string(),
        }),
        username_hash: identity.username_hash,
    })
}
