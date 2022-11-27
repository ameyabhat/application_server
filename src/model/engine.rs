use std::collections::HashMap;

use sqlx::PgPool;

use uuid::Uuid;

use crate::{
    db::{self},
    endpoints::errors::ModelError,
};

use super::types::Applicant;

pub async fn get_applicants(
    pool: PgPool,
    applicants: &[String],
) -> Result<Vec<Applicant>, ModelError> {
    match db::transactions::get_applicants_db(&pool, applicants).await {
        Ok(vec) => Ok(vec
            .iter()
            .map(|(nuid, name, reg_time, sub_time, ok)| {
                let time_to_completion = match sub_time.signed_duration_since(*reg_time).to_std() {
                    Ok(d) => d,
                    Err(_) => std::time::Duration::ZERO,
                };
                Applicant {
                    nuid: nuid.clone(),
                    name: name.clone(),
                    time_to_completion,
                    ok: *ok,
                }
            })
            .collect()),
        Err(_) => Err(ModelError::SqlError),
    }
}
pub async fn register_user(
    pool: PgPool,
    name: String,
    nuid: String,
) -> Result<(Uuid, String), ModelError> {
    let token = Uuid::new_v4();
    let challenge_str = generate_challenge_string();
    let soln = find_kmers(&challenge_str, 3);

    match db::transactions::register_user_db(&pool, token, name, nuid, &challenge_str, soln).await {
        Ok(()) => Ok((token, challenge_str)),
        // there's a bunch of different ways that this can fail, I should probably
        // handle the error -
        Err(_e) => Err(ModelError::DuplicateUser),
    }
}

pub async fn retreive_token(pool: PgPool, nuid: &String) -> Result<Uuid, ModelError> {
    match db::transactions::retreive_token_db(&pool, nuid).await {
        Ok(token) => Ok(token),
        Err(_) => Err(ModelError::NoUserFound),
    }
}

pub async fn retreive_challenge(pool: &PgPool, token: Uuid) -> Result<String, ModelError> {
    match db::transactions::retreive_challenge_db(pool, token).await {
        Ok(challenge) => Ok(challenge),
        Err(_) => Err(ModelError::NoUserFound),
    }
}

pub async fn check_solution(
    pool: PgPool,
    token: Uuid,
    given_soln: &HashMap<String, u64>,
) -> Result<(bool, HashMap<String, u64>), ModelError> {
    // Check if the solution is correct - write the row to the solutions table
    match db::transactions::retreive_soln(&pool, token).await {
        Ok((soln, nuid)) => {
            let ok = soln == *given_soln;
            if let Err(e) = db::transactions::write_submission(pool, nuid, ok).await {
                panic!("We failed to write the submission properly: {}", e)
            }
            Ok((ok, soln))
        }
        Err(_) => Err(ModelError::NoUserFound),
    }
}

fn generate_challenge_string() -> String {
    let charset = "ACTG";
    random_string::generate(100, charset)
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
        assert!(generate_challenge_string().len() == 100);
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
