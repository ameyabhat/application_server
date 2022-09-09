use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::reject;

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    DuplicateUser,
    IncorrectSolution {
        expected_solution: HashMap<String, u64>,
        given_solution: HashMap<String, u64>,
    },
    DeserializeError,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum ModelError {
    #[error("Incorrect solution")]
    IncorrectSolution {
        expected_solution: HashMap<String, u64>,
        given_solution: HashMap<String, u64>,
    },
    #[error("A registration with this NUID exists")]
    DuplicateUser,
}

impl reject::Reject for ModelError {}
