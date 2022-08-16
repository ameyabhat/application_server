use std::{collections::HashMap, io::Error};

use sqlx::PgPool;
use uuid::Uuid;

use crate::db;
pub struct Model {
    pub pool: PgPool,
}

impl Model {
    pub async fn register_user(&mut self, name: String, nuid: String) -> Result<String, Error> {
        let token = Uuid::new_v4();

        match db::transactions::register_user_db(pool) {}
        Ok("".to_owned())
    }

    pub fn retreive_token(nuid: String) -> Result<String, Error> {
        Ok("".to_owned())
    }

    pub fn generate_challenge_string() -> String {
        "".to_owned()
    }

    // Return the kmers as a map from strings of length k to
    pub fn find_kmers(challenge_str: String, k: u64) -> HashMap<String, u64> {
        HashMap::new()
    }
}
