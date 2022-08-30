use rand::distributions::{Alphanumeric, DistString};
use std::{collections::HashMap, io::Error};

use sqlx::PgPool;

use uuid::Uuid;

use crate::db::{self};

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("A registration with this NUID exists")]
    DuplicateUser,
}

pub async fn register_user(
    pool: PgPool,
    name: String,
    nuid: String,
) -> Result<(Uuid, String), DatabaseError> {
    let token = Uuid::new_v4();
    let challenge_str = generate_challenge_string();
    let soln = find_kmers(&challenge_str, 3);

    match db::transactions::register_user_db(
        &pool,
        token,
        name,
        nuid,
        generate_challenge_string(),
        soln,
    )
    .await
    {
        Ok(_) => Ok((token, challenge_str)),
        Err(_) => Err(DatabaseError::DuplicateUser),
    }
}

pub async fn retreive_token(pool: PgPool, nuid: String) -> Result<Uuid, Error> {
    match db::transactions::retreive_token_db(&pool, nuid).await {
        Ok(token) => Ok(token),
        Err(_) => todo!(
            "Figure out how to handle the retreive token err properly - this one actually matters"
        ),
    }
}

pub async fn check_solution(
    pool: PgPool,
    token: Uuid,
    given_soln: &HashMap<String, u64>,
) -> Result<(bool, HashMap<String, u64>), Error> {
    // Check if the solution is correct - write the row to the solutions table
    match db::transactions::retreive_soln(&pool, token).await {
        Ok((soln, solution_id)) => {
            let ok = soln == *given_soln;
            if let Err(e) = db::transactions::write_submission(pool, solution_id, token, ok).await {
                panic!("We failed to write the submission properly: {}", e)
            }
            Ok((ok, soln))
        }
        Err(_) => {
            panic!("We failed to retreive the soln - it's possible this user never registered")
        }
    }
}

fn generate_challenge_string() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 100)
}

// Return the kmers as a map from strings of length k to
fn find_kmers(challenge_str: &String, k: usize) -> HashMap<String, u64> {
    let mut start_ind = 0;
    let mut soln: HashMap<String, u64> = HashMap::new();
    while start_ind + k <= challenge_str.len() {
        let slice = &challenge_str[start_ind..start_ind + k];
        soln.entry(slice.to_string())
            .and_modify(|kmer_count| *kmer_count += 1)
            .or_insert(1);
        start_ind += 1;
    }

    soln
}

#[cfg(test)]
mod tests {
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
        #[macro_export]
        macro_rules! fuck_your_strings {
            ($(($key:expr, $value: expr),)+) => {
                {
                    let mut map = std::collections::HashMap::new();
                    $(
                        map.insert(String::from($key), $value);
                    )*
                    map
                }
            };
        }

        let long_challenge_string = &String::from("aabbceedeaab");

        let soln = find_kmers(long_challenge_string, 3);
        let correct_soln = fuck_your_strings!(
            ("aab", 2),
            ("abb", 1),
            ("bbc", 1),
            ("bce", 1),
            ("cee", 1),
            ("eed", 1),
            ("ede", 1),
            ("dea", 1),
            ("eaa", 1),
        );

        assert_eq!(soln, correct_soln);
        Ok(())
    }
}
