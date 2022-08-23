use serde_derive::{Deserialize, Serialize};
use warp::{reply, Filter, Rejection, Reply};

use super::routes::{forgot_token_route, register_route};

use crate::model::Model;

#[derive(Clone)]
pub struct Handler<'a> {
    model: &'a Model,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    token: String,
}
#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    name: String,
    nuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct HandleForgotTokenResponse {
    token: String,
}

type WarpResponse = Result<impl Reply, Rejection>;

pub fn end(handler: Handler) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let register_user_wrapper = {
        let handler2 = handler.clone();
        |req| handler.handle_register(req)
    };

    register_route().and_then(register_user_wrapper)
    //.or(forgot_token_route().and_then(|nuid| self.handle_forgot_token(nuid)))
}

impl<'a> Handler<'a> {
    pub fn end(&'a self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let handle_register_wrapper = move |req: RegisterRequest| self.handle_register(req);
        register_route().and_then(handle_register_wrapper)
    }
    pub async fn handle_register(&self, request: RegisterRequest) -> WarpResponse {
        info!(
            "registering user {}, with nuid {}",
            request.name, request.nuid
        );

        let token = match self.model.register_user(request.name, request.nuid).await {
            Ok(token) => token.to_string(),
            Err(_err) => todo!("Send back failure"),
        };
        Ok(reply::json(&RegisterResponse { token }))
    }

    pub async fn handle_forgot_token(&self, nuid: String) -> Result<impl Reply, Rejection> {
        let token = match self.model.retreive_token(nuid).await {
            Ok(token) => token.to_string(),
            Err(e) => todo!("send back no user found failure"),
        };

        Ok(reply::json(&HandleForgotTokenResponse { token }))
    }
}
