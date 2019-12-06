use crate::umbra_auth::server::UmbraAuth;

use crate::handler::{
    is_alive,
    registration,
    system,
};

pub struct Umbra {
    pub db: crate::di::DBContainer,
    pub mysql: crate::di::MySqlContainer,
}

#[tonic::async_trait]
impl UmbraAuth for Umbra {
    async fn is_alive(&self, _request: is_alive::Request) -> is_alive::Reply {
        is_alive::check()
    }

    async fn identity_register(&self, request: registration::Request) -> registration::Reply {
        use crate::umbra_auth::{
            identity_reply::Outcome,
            IdentityReply,
        };

        let identity = registration::register(self.db.clone(), &request.into_inner())?;

        let reply = IdentityReply {
            outcome: Some(Outcome::Ok(identity)),
        };

        Ok(tonic::Response::new(reply))
    }

    async fn system_search(&self, request: system::SearchRequest) -> system::SearchReply {

        let request = request.into_inner();

        let (limit, offset) = request.page.map_or((100, 0), |page| {
            let limit = page.limit;
            let offset = limit * page.number;

            (limit, offset)
        });
        let slug = &request.slug;

        let reply =
            system::search(self.mysql.clone(), limit, offset, slug).await?;

        Ok(tonic::Response::new(reply))
    }
}
