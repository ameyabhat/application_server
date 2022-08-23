use serde_derive::{Deserialize, Serialize};
use warp::{reply, Filter, Rejection, Reply};

use super::routes::{forgot_token_route, register_route};

use crate::model::Model;

#[derive(Clone)]
pub struct Server<'a> {
    model: &'a Model<'a>,
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

impl<'b> Server<'b> {
    pub fn end<'a>(&'a self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone + 'a {
        //let handle_register_wrapper = move |req: RegisterRequest| self.handle_register(req);
        register_route().and_then(Server::h)
    }
    pub async fn h(request: RegisterRequest) -> WarpResponse {
        Ok(reply::json(&RegisterResponse {
            token: "tok".to_owned(),
        }))
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
            Err(_e) => todo!("send back no user found failure"),
        };

        Ok(reply::json(&HandleForgotTokenResponse { token }))
    }
}
