use super::messages::{HandleForgotTokenResponse, RegisterRequest, RegisterResponse};
use super::routes::{forgot_token_route, health, register_route, with_db};
use crate::model::{register_user, retreive_token};
use serde_json::json;
use sqlx::PgPool;
use warp::{reply, Filter, Rejection, Reply};

type WarpResponse = Result<impl Reply, Rejection>;

pub fn end(o: Option<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    register_route()
        .and(with_db(o.clone()))
        .and_then(handle_register)
        .or(forgot_token_route()
            .and(with_db(o))
            .and_then(handle_forgot_token)
            .or(health().and_then(health_check)))
}

pub async fn health_check() -> WarpResponse {
    Ok(reply::json(&json!({
        "healthy": true
    })))
}

pub async fn handle_register(request: RegisterRequest, p: PgPool) -> WarpResponse {
    info!(
        "registering user {}, with nuid {}",
        request.name, request.nuid
    );

    let token = match register_user(p, request.name, request.nuid).await {
        Ok(token) => token.to_string(),
        Err(err) => {
            panic!("failed to register user: {}", err)
        }
    };
    Ok(reply::json(&RegisterResponse { token }))
}

pub async fn handle_forgot_token(nuid: String, p: PgPool) -> Result<impl Reply, Rejection> {
    let token = match retreive_token(p, nuid).await {
        Ok(token) => token.to_string(),
        Err(_e) => todo!("send back no user found failure"),
    };

    Ok(reply::json(&HandleForgotTokenResponse { token }))
}
