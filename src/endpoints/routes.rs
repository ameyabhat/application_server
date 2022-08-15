use serde_derive::{Deserialize, Serialize};
use warp::filters::BoxedFilter;
use warp::{path, Filter, Reply};

use super::handlers;

pub fn register_route() -> BoxedFilter<(handlers::RegisterRequest,)> {
    warp::post()
        .and(path("register"))
        .and(path::end())
        .and(warp::body::json())
        .boxed()
}

pub fn forgot_token_route() -> BoxedFilter<(String,)> {
    let ftr = warp::path!("forgot_token" / String);

    warp::get().and(ftr).boxed()
}
