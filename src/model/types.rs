use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Applicant {
    pub time_to_completion: Duration,
    pub ok: bool,
    pub name: String,
    pub nuid: String,
}
