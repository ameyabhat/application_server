use std::collections::HashMap;
use std::convert::Infallible;

use super::messages::{
    ErrorResponse, HandleForgotTokenResponse, RegisterRequest, RegisterResponse,
};
use super::routes::{forgot_token_route, health, register_route, submit, with_db};
use crate::endpoints::ApiError;
use crate::model::{check_solution, register_user, retreive_token};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use warp::hyper::StatusCode;
use warp::{reject, reply, Filter, Rejection, Reply};

type WarpResponse = Result<impl Reply, Rejection>;

/*
   The expansion should look something like
   handle!(route, handler) => {
    route().and(with_db(o.clone)).and_then(handler)
   }
*/

#[macro_export]
macro_rules! handle_with_db {
    ($route:expr, $db:expr, $handler:expr) => {
        $route().and(with_db($db.clone())).and_then($handler)
    };
}

#[macro_export]
macro_rules! api_err {
    ($msg:expr, $api_err:expr) => {
        $crate::endpoints::messages::ErrorResponse {
            msg: $msg,
            error: Some($api_err),
        }
    };
    ($msg:expr) => {
        $crate::endpoints::messages::ErrorResponse {
            msg: $msg,
            error: None,
        }
    };
}

pub fn end(o: Option<PgPool>) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone {
    handle_with_db!(register_route, o, handle_register)
        .or(handle_with_db!(forgot_token_route, o, handle_forgot_token))
        .or(handle_with_db!(submit, o, handle_submit))
        .or(health().and_then(health_check))
        .recover(handle_rejection)
}

pub async fn handle_register(request: RegisterRequest, p: PgPool) -> WarpResponse {
    info!(
        "registering user {}, with nuid {}",
        request.name, request.nuid
    );

    match register_user(p, request.name, request.nuid).await {
        Ok((token, challenge_string)) => Ok(reply::json(&RegisterResponse {
            token: token.to_string(),
            challenge_string,
        })),
        // Actually send back an error here you fucking muppet
        // Should be a 409 conflict error if the error doesnt exist,
        Err(_) => Err(reject::custom(ApiError::DuplicateUser)),
    }
}

//pub async fn get_challenge_string(_token: String) -> WarpResponse {}
// On error, send back a 400
pub async fn handle_submit(token: Uuid, soln: HashMap<String, u64>, p: PgPool) -> WarpResponse {
    // Depending on what check solution does, either return a reply json or a rejection
    match check_solution(p, token, &soln).await {
        Ok((is_correct, expected_soln)) => {
            if is_correct {
                Ok(reply::json(&"Correct! Nice work".to_string()))
            } else {
                Err(reject::custom(ApiError::IncorrectSolution {
                    expected_solution: expected_soln,
                    given_solution: soln.clone(),
                }))
            }
        }
        Err(_) => panic!("uh i fucked up"),
    }
}

pub async fn handle_forgot_token(nuid: String, p: PgPool) -> Result<impl Reply, Rejection> {
    let token = match retreive_token(p, nuid).await {
        Ok(token) => token.to_string(),
        Err(_e) => todo!("send back no user found failure"),
    };

    Ok(reply::json(&HandleForgotTokenResponse { token }))
}

pub async fn health_check() -> WarpResponse {
    Ok(reply::json(&json!({
        "healthy": true
    })))
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let msg: ErrorResponse;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        msg = api_err!("The path you're trying to hit doesn't exist");
    } else if let Some(wrapped_err) = err.find::<ApiError>() {
        match wrapped_err {
            ApiError::DuplicateUser => {
                msg = api_err!("This NUID has already been used to register");
                code = StatusCode::CONFLICT;
            }
            ApiError::IncorrectSolution {
                expected_solution,
                given_solution,
            } => {
                msg = api_err!(
                    "Incorrect solution",
                    ApiError::IncorrectSolution {
                        expected_solution: expected_solution.clone(),
                        given_solution: given_solution.clone()
                    }
                );
                code = StatusCode::BAD_REQUEST;
            }
        }
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        msg = api_err!("UNHANDLED_REJECTION");
    }

    Ok(reply::with_status(reply::json(&msg), code))
}
