use crate::umbra_model::errors::UmbraModelError as UME;

#[derive(Debug)]
pub enum UmbraGrpcError {
  Internal(String),
  NotFound,
}

impl From<UME> for UmbraGrpcError {
  fn from(error: UME) -> Self {
    match error {
      UME::CryptoError(message) => Self::Internal(message),
      UME::Failure(message) => Self::Internal(message),
      UME::NotFound => Self::NotFound,
      UME::Rollback => {
        Self::Internal(String::from("Transaction failed and was rolled back"))
      }
    }
  }
}

impl From<UmbraGrpcError> for tonic::Status {
  fn from(error: UmbraGrpcError) -> Self {
    use tonic::{Code, Status};

    match error {
      UmbraGrpcError::Internal(message) => {
        Status::new(Code::Internal, &message)
      }
      UmbraGrpcError::NotFound => Status::new(Code::NotFound, "NOT_FOUND"),
    }
  }
}
