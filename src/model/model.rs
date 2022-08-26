use rand::distributions::{Alphanumeric, DistString};
use std::{collections::HashMap, io::Error};

use sqlx::PgPool;

use uuid::Uuid;

use crate::db::{self, transactions::retreive_token_db};

pub async fn register_user(pool: PgPool, name: String, nuid: String) -> Result<Uuid, Error> {
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
        Ok(_) => Ok(token),
        Err(e) => todo!("Figure out how to handle the db error properly: {}", e),
    }
}

pub async fn retreive_token(pool: PgPool, nuid: String) -> Result<Uuid, Error> {
    match retreive_token_db(&pool, nuid).await {
        Ok(token) => Ok(token),
        Err(_) => todo!(
            "Figure out how to handle the retreive token err properly - this one actually matters"
        ),
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
