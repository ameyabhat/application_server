use super::endpoints::handlers::{handle_forgot_token, handle_register};
use super::endpoints::routes::{forgot_token_route, register_route};
use warp::{Filter, Rejection, Reply};

pub fn end() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    register_route()
        .and_then(handle_register)
        .or(forgot_token_route().and_then(handle_forgot_token))
}
