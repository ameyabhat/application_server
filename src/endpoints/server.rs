use super::messages::{HandleForgotTokenResponse, RegisterRequest, RegisterResponse};
use super::routes::{forgot_token_route, register_route, with_db};
use crate::model::{register_user, retreive_token};
use sqlx::PgPool;
use warp::{reply, Filter, Rejection, Reply};

type WarpResponse = Result<impl Reply, Rejection>;

pub fn end(o: Option<PgPool>) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    register_route()
        .and(with_db(o.clone()))
        .and_then(handle_register)
        .or(forgot_token_route()
            .and(with_db(o))
            .and_then(handle_forgot_token))
    //register_route().and_then(reg)
}
pub async fn handle_register(request: RegisterRequest, p: PgPool) -> WarpResponse {
    info!(
        "registering user {}, with nuid {}",
        request.name, request.nuid
    );

    let token = match register_user(p, request.name, request.nuid).await {
        Ok(token) => token.to_string(),
        Err(_err) => todo!("Send back failure"),
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
