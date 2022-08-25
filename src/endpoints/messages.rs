use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterResponse {
    pub token: String,
}
#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub nuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct HandleForgotTokenResponse {
    pub token: String,
}
