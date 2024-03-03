use std::collections::HashMap;
use std::convert::Infallible;

use sqlx::PgPool;
use uuid::Uuid;
use warp::filters::BoxedFilter;
use warp::{path, Filter, Rejection};

use super::messages::RegisterRequest;

pub fn register_route() -> BoxedFilter<(RegisterRequest,)> {
    let register = warp::path!("register");
    warp::post().and(register).and(warp::body::json()).boxed()
}

pub fn forgot_token_route() -> BoxedFilter<(String,)> {
    let ftr = warp::path!("forgot_token" / String);

    warp::get().and(ftr).boxed()
}

pub fn health() -> BoxedFilter<()> {
    let health = warp::path!("health");
    warp::get().and(health).boxed()
}

pub fn submit() -> BoxedFilter<(Uuid, HashMap<String, u64>)> {
    let route = warp::path!("submit" / Uuid);
    warp::post().and(route).and(warp::body::json()).boxed()
}

pub fn get_challenge_string_route() -> BoxedFilter<(Uuid,)> {
    let route = warp::path!("challenge" / Uuid);

    warp::get().and(route).boxed()
}

/*
This route should return:
   - whether or not the applicant provided the correct solution
   - the time elapsed between registration and the first succesful entry
*/
pub fn get_applicant_route() -> BoxedFilter<(String,)> {
    let route = path!("applicant" / String);

    warp::get().and(route).boxed()
}

pub fn get_applicants_route() -> BoxedFilter<(Vec<String>,)> {
    let route = path!("applicants");

    warp::get().and(route).and(warp::body::json()).boxed()
}

// All this does is include the db pool in scope, it shouldn't change the actual route
pub fn with_db(o: Option<PgPool>) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || {
        // This is is fine b/c a PgPool is just a reference counted
        // pointer to an inner db. The implementation uses Arc
        o.clone()
        // let o = o.clone();
        // {
        //     if let Some(pool) = o {
        //         Ok(pool)
        //     } else {
        //         Err(warp::reject::not_found())
        //     }
        // }
    })
}
