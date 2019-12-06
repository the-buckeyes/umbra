use crate::umbra_auth::server::UmbraAuth;

use crate::handler::{
    is_alive,
    registration,
};

pub struct Umbra {
    pub db: crate::di::DBContainer,
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
}
