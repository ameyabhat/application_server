use std::collections::HashMap;
use std::convert::Infallible;

use super::errors::ModelError;
use super::messages::{
    ErrorResponse, GetChallengeString, HandleForgotTokenResponse, RegisterRequest, RegisterResponse,
};
use super::routes::{
    forgot_token_route, get_applicant_route, get_applicants_route, get_challenge_string_route,
    health, register_route, submit, with_db,
};
use crate::endpoints::ApiError;
use crate::model::{
    check_solution, get_applicants, register_user, retreive_challenge, retreive_token,
};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use warp::body::BodyDeserializeError;
use warp::hyper::StatusCode;
use warp::reject::MethodNotAllowed;
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
        .or(handle_with_db!(
            get_challenge_string_route,
            o,
            handle_get_challenge
        ))
        .or(health().and_then(health_check))
        .or(handle_with_db!(
            get_applicant_route,
            o,
            handle_get_applicant
        ))
        .or(handle_with_db!(
            get_applicants_route,
            o,
            handle_get_applicants
        ))
        .recover(handle_rejection)
}

// This is weird - if I use the WarpResult alias here, it forces me to use the same
// concrete return types as the other functions that use the WarpResult alias
// def because using `impl trait` syntax in aliases is experimental and on nightly
// should switch away from nightly - it'll make deployment more stable as well
pub async fn handle_get_applicant(nuid: String, p: PgPool) -> Result<impl Reply, Rejection> {
    // look up the applicant
    match get_applicants(p, &[nuid]).await {
        Ok(applicant) => {
            let code;
            if applicant.len() == 1 {
                code = StatusCode::OK;
                Ok(reply::with_status(reply::json(&applicant[0]), code))
            } else if applicant.is_empty() {
                code = StatusCode::NOT_FOUND;
                Ok(reply::with_status(
                    reply::json(&"Could not find submission from applicant"),
                    code,
                ))
            } else {
                let code = StatusCode::INTERNAL_SERVER_ERROR;
                let msg = api_err!("Fetched the wrong number of applicants somehow ¯\\_(ツ)_/¯  ");
                Ok(reply::with_status(reply::json(&msg), code))
            }
        }
        // This will just bubble down to a 500 which seems super reasonable
        // Assuming that this is a sql error - no other reason that this would fail
        Err(_) => Err(reject::custom(ModelError::SqlError)),
    }
}

pub async fn handle_get_applicants(nuids: Vec<String>, p: PgPool) -> WarpResponse {
    match get_applicants(p, &nuids).await {
        Ok(applicants) => {
            if applicants.len() == nuids.len() {
                Ok(reply::json(&applicants))
            } else {
                let mut applicants_not_found: Vec<String> = nuids.clone();
                // ok so basically we copy the nuids to a new list,
                // then keep only the nuids that do not correspond to any of the
                // applicants we fetched. This means that we only get the applicants
                // that aren't found
                // worst case O((n/2)^2) - polynomial so we're fine
                applicants_not_found
                    .retain(|nuid| !applicants.iter().any(|applicant| applicant.nuid == *nuid));

                Err(reject::custom(ModelError::ApplicantsNotFound {
                    applicants_found: applicants,
                    applicants_not_found,
                }))
            }
        }
        Err(_) => todo!("falling through here"),
    }
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
        Err(e) => Err(reject::custom(e)),
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
                Err(reject::custom(ModelError::IncorrectSolution {
                    expected_solution: expected_soln,
                    given_solution: soln.clone(),
                }))
            }
        }
        Err(e) => Err(reject::custom(e)),
    }
}

pub async fn handle_forgot_token(nuid: String, p: PgPool) -> Result<impl Reply, Rejection> {
    match retreive_token(p, nuid).await {
        Ok(token) => Ok(reply::json(&HandleForgotTokenResponse {
            token: token.to_string(),
        })),
        Err(e) => Err(reject::custom(e)),
    }
}

pub async fn health_check() -> WarpResponse {
    Ok(reply::json(&json!({
        "healthy": true
    })))
}

pub async fn handle_get_challenge(token: Uuid, pool: PgPool) -> WarpResponse {
    match retreive_challenge(&pool, token).await {
        Ok(challenge_string) => Ok(reply::json(&GetChallengeString { challenge_string })),
        Err(e) => Err(reject::custom(e)),
    }
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let msg: ErrorResponse;

    if let Some(wrapped_err) = err.find::<ModelError>() {
        match wrapped_err {
            ModelError::DuplicateUser => {
                msg = api_err!("This NUID has already been used to register");
                code = StatusCode::CONFLICT;
            }
            ModelError::IncorrectSolution {
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
            ModelError::ApplicantsNotFound {
                applicants_found,
                applicants_not_found,
            } => {
                code = StatusCode::NOT_FOUND;
                msg = api_err!(
                    "Submissions from one or more the applicants requested was not found",
                    ApiError::ApplicantsNotFound {
                        applicants_found: applicants_found.to_vec(),
                        applicants_not_found: applicants_not_found.clone()
                    }
                )
            }
            ModelError::SqlError => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                msg = api_err!("SQL Error - text me if this happens");
                warn!("{:?}", err)
            }
            ModelError::NoUserFound => {
                code = StatusCode::NOT_FOUND;
                msg = api_err!("No user with this token or nuid exists")
            }
        }
    } else if err.find::<BodyDeserializeError>().is_some() {
        code = StatusCode::BAD_REQUEST;
        msg = api_err!("Bad request - check your request body")
    }
    // This is super jank - we're mapping a 405 to a 404
    // This issue explains why: https://github.com/seanmonstar/warp/issues/77
    // I'll fix this eventually, I need to fix the library
    // fucking warp man
    // This shit sucks - for some reason post request are being logged as
    // methodNotAllowed
    else if err.find::<MethodNotAllowed>().is_some() {
        code = StatusCode::NOT_FOUND;
        msg = api_err!(
            "The path you're trying to hit doesn't exist - check your endpoints and your request method"
        );
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        msg = api_err!("UNHANDLED_REJECTION");
        warn!("{:?}", err)
    }

    Ok(reply::with_status(reply::json(&msg), code))
}

mod tests {
    // We'll write API tests here eventually - i wonder if I can write service tests
    // using docker compose. Should probably figure out how to do that for generate

    // You can just mock the DB
}
