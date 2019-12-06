use umbra_model;

mod di;
mod errors;
mod handler;
mod service;
mod umbra_auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let port = 47770;
  let host = "[::1]";

  let addr = format!("{}:{}", host, port).parse()?;

  let pool = umbra_model::db::connect();
  let service = service::Umbra {
      db: std::sync::Arc::new(pool),
  };

  tonic::transport::Server::builder()
    .add_service(umbra_auth::server::UmbraAuthServer::new(service))
    .serve(addr)
    .await?;

  Ok(())
}
