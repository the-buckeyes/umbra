use crate::errors::UmbraGrpcError;
use crate::umbra_auth::{ListMeta, Page, System, SystemList, SystemSearchRequest};
use umbra_model::mysql::{connection as db, system::System as SystemModel};

pub type SearchRequest = tonic::Request<SystemSearchRequest>;

pub type SearchReply = Result<tonic::Response<SystemList>, tonic::Status>;

pub async fn search(
    db: crate::di::MySqlContainer,
    limit: i32,
    offset: i32,
    _slug: &str,
) -> Result<SystemList, UmbraGrpcError> {
    let db = db::acquire(db).await?;
    let (_, model_list) = SystemModel::list(db).await?;

    let system_list = model_list.into_iter().map(|model| System {
        id: model.id.to_string(),
        slug: model.slug,
        display: model.display,
        organizations: [].to_vec(),
    });

    let number = if offset > 0 { offset / limit } else { 0 };

    let reply = SystemList {
        meta: Some(ListMeta {
            total: 0,
            page: Some(Page {
                limit,
                number,
                total: 0,
            }),
        }),
        systems: system_list.collect(),
    };

    Ok(reply)
}
