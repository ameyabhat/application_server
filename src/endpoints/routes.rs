use warp::filters::BoxedFilter;
use warp::{path, Filter};

use super::server;

pub fn register_route() -> BoxedFilter<(server::RegisterRequest,)> {
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
