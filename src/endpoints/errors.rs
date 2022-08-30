use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::reject;

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum ApiError {
    #[error("A registration with this NUID exists")]
    DuplicateUser,
    #[error("Incorrect solution")]
    #[serde(rename = "incorrect_solution")]
    IncorrectSolution {
        expected_solution: HashMap<String, u64>,
        given_solution: HashMap<String, u64>,
    },
}

impl reject::Reject for ApiError {}
