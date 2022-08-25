use sqlx::PgPool;
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection};

use super::messages::RegisterRequest;

pub fn register_route() -> BoxedFilter<(RegisterRequest,)> {
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

pub fn with_db(o: Option<PgPool>) -> impl Filter<Extract = (PgPool,), Error = Rejection> + Clone {
    warp::any().and_then(move || {
        // This is is fine b/c a PgPool is just a reference counted
        // pointer to an inner db. The implementation uses Arc
        let o = o.clone();
        async move {
            if let Some(pool) = o {
                Ok(pool)
            } else {
                Err(warp::reject::not_found())
            }
        }
    })
}
