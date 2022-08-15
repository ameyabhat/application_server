use super::endpoints::register::handle_register;
use super::endpoints::routes::register_route;
use warp::{Filter, Rejection, Reply};

pub fn end() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    register_route().and_then(handle_register)
}
