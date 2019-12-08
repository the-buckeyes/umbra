use crate::umbra_auth::{is_alive_reply::Status as IsAliveStatus, IsAliveReply, IsAliveRequest};

pub type Request = tonic::Request<IsAliveRequest>;

pub type Reply = Result<tonic::Response<IsAliveReply>, tonic::Status>;

pub fn check() -> Reply {
    let reply = IsAliveReply {
        status: IsAliveStatus::Alive as i32,
    };
    Ok(tonic::Response::new(reply))
}
