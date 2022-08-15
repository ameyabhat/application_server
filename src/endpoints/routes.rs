use warp::filters::BoxedFilter;
use warp::{path, Filter, Reply};

pub fn register_route() -> BoxedFilter<()> {
    warp::post().and(path("register")).and(path::end()).boxed()
}
