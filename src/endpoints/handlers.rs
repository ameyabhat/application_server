use serde_derive::{Deserialize, Serialize};
use warp::{reply, Rejection, Reply};

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    token: String,
}
#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    name: String,
    nuid: String,
}

pub async fn handle_register(request: RegisterRequest) -> Result<impl Reply, Rejection> {
    info!(
        "registering user {}, with nuid {}",
        request.name, request.nuid
    );
    Ok(reply::json(&RegisterResponse {
        token: request.name,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct HandleForgotTokenResponse {
    token: String,
}

pub async fn handle_forgot_token(nuid: String) -> Result<impl Reply, Rejection> {
    Ok(reply::json(&HandleForgotTokenResponse {
        token: "the token you forgot".to_owned(),
    }))
}
