use serde_derive::{Deserialize, Serialize};
use warp::{reply, Rejection, Reply};

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
    token: String,
}

pub async fn handle_register() -> Result<impl Reply, Rejection> {
    Ok(reply::json(&RegisterResponse {
        token: "test Token".to_owned(),
    }))
}
