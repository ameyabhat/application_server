use rand::distributions::{Alphanumeric, DistString};
use std::{collections::HashMap, io::Error};

use sqlx::PgPool;

use uuid::Uuid;

use crate::db;
pub struct Model {
    pub pool: PgPool,
}

impl Model {
    pub async fn register_user(self, name: String, nuid: String) -> Result<String, Error> {
        let token = Uuid::new_v4();

        match db::transactions::register_user_db(
            self.pool,
            token,
            name,
            nuid,
            generate_challenge_string(),
        )
        .await
        {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
        Ok("".to_owned())
    }

    pub fn retreive_token(nuid: String) -> Result<String, Error> {
        Ok("".to_owned())
    }
}

pub fn generate_challenge_string() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 100)
}

// Return the kmers as a map from strings of length k to
pub fn find_kmers<'a>(challenge_str: &String, k: usize) -> HashMap<&str, u64> {
    let mut start_ind = 0;
    let mut soln: HashMap<&str, u64> = HashMap::new();
    while start_ind + k <= challenge_str.len() {
        let slice = &challenge_str[start_ind..start_ind + k];
        soln.entry(slice)
            .and_modify(|kmer_count| *kmer_count += 1)
            .or_insert(1);
        start_ind += 1;
    }

    soln
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::Error;

    use super::find_kmers;
    use super::generate_challenge_string;

    #[test]
    fn test_rand_str() -> Result<(), Error> {
        println!("{}", generate_challenge_string());
        Ok(())
    }

    #[test]
    fn test_empty_challenge_string() -> Result<(), Error> {
        let empty_challenge_string = &String::from("");
        let empty_soln = find_kmers(empty_challenge_string, 3);
        assert!(empty_soln.is_empty());

        Ok(())
    }

    #[test]
    fn test_challenge_string_too_small() -> Result<(), Error> {
        let small_challenge_string = &String::from("ab");
        let empty_soln = find_kmers(small_challenge_string, 3);
        assert!(empty_soln.is_empty());

        Ok(())
    }

    #[test]
    fn test_long_challenge_string() -> Result<(), Error> {
        let long_challenge_string = &String::from("aabbceedeaab");

        let soln = find_kmers(long_challenge_string, 3);

        let correct_soln = HashMap::from([
            ("aab", 2),
            ("abb", 1),
            ("bbc", 1),
            ("bce", 1),
            ("cee", 1),
            ("eed", 1),
            ("ede", 1),
            ("dea", 1),
            ("eaa", 1),
        ]);

        assert_eq!(soln, correct_soln);
        Ok(())
    }
}
