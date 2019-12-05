use crate::umbra_auth::server::UmbraAuth;

use crate::handler::{
    is_alive,
};

pub struct Umbra {
    pub db: std::sync::Arc<umbra_model::db::MyPool>,
}

#[tonic::async_trait]
impl UmbraAuth for Umbra {
    async fn is_alive(&self, _request: is_alive::Request) -> is_alive::Reply {
        is_alive::check()
    }
}
