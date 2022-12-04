use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::reject;

use crate::model::types::Applicant;

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    DuplicateUser,
    IncorrectSolution {
        given_solution: HashMap<String, u64>,
    },
    DeserializeError,
    ApplicantsNotFound {
        applicants_found: Vec<Applicant>,
        applicants_not_found: Vec<String>,
    },
    NoUserFound,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum ModelError {
    #[error("Incorrect solution")]
    IncorrectSolution {
        given_solution: HashMap<String, u64>,
    },
    #[error("A registration with this NUID exists")]
    DuplicateUser,
    #[error("One or more of the applicants requested not found")]
    ApplicantsNotFound {
        applicants_found: Vec<Applicant>,
        applicants_not_found: Vec<String>,
    },
    #[error("SQL error")]
    SqlError,
    #[error("No user with this token exists")]
    NoUserFound,
}

impl reject::Reject for ModelError {}
