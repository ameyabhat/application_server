use chrono::{DateTime, Utc};
use core::panic;
use serde_json;
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

use sqlx::{query, PgPool};

pub async fn register_user_db(
    pool: &PgPool,
    token: Uuid,
    name: String,
    nuid: String,
    challenge_string: &String,
    solution: HashMap<String, u64>,
) -> Result<(), sqlx::Error> {
    // Insert the applicant
    let registration_time: DateTime<Utc> = SystemTime::now().into();
    let ser_solution = match serde_json::to_value(&solution) {
        Ok(val) => val,
        Err(_) => todo!("Figure out how to handle the serde error properly"),
    };

    query!(
        r#"INSERT INTO applicants (nuid, applicant_name, registration_time, token, challenge_string, solution)
         VALUES ($1, $2, $3, $4, $5, $6);"#,
        nuid,
        name,
        registration_time,
        token,
        challenge_string,
        ser_solution
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_applicants_db(
    pool: &PgPool,
    nuids: &[String],
) -> Result<Vec<(String, String, DateTime<Utc>, DateTime<Utc>, bool)>, sqlx::Error> {
    // This is a hack, sqlx doesn't support vector replacement into an IN statement
    let records = query!(
        r#"SELECT DISTINCT ON (nuid) nuid, applicant_name, ok, submission_time, 
        registration_time FROM submissions JOIN applicants using(nuid) where 
        nuid=ANY($1) ORDER BY nuid, submission_time DESC;"#,
        &nuids[..]
    )
    .fetch_all(pool)
    .await?;

    Ok(records
        .iter()
        .map(|record| {
            (
                record.nuid.clone(),
                record.applicant_name.clone(),
                record.registration_time,
                record.submission_time,
                record.ok,
            )
        })
        .collect())
}

pub async fn retreive_token_db(pool: &PgPool, nuid: &String) -> Result<Uuid, sqlx::Error> {
    let record = query!(r#"SELECT token FROM applicants WHERE nuid=$1"#, nuid)
        .fetch_one(pool)
        .await?;

    Ok(record.token)
}

pub async fn retreive_challenge_db(pool: &PgPool, token: Uuid) -> Result<String, sqlx::Error> {
    let record = query!(
        r#"select challenge_string from applicants where token=$1"#,
        token
    )
    .fetch_one(pool)
    .await?;

    Ok(record.challenge_string)
}

pub async fn retreive_soln(
    pool: &PgPool,
    token: Uuid,
) -> Result<(HashMap<String, u64>, String), sqlx::Error> {
    let record = query!(
        r#"SELECT nuid, solution FROM applicants WHERE token=$1"#,
        token
    )
    .fetch_one(pool)
    .await?;

    match serde_json::from_value(record.solution) {
        Ok(soln) => Ok((soln, record.nuid)),
        Err(_e) => panic!("solution didn't deserialize properly - this should never happen"),
    }
}

pub async fn write_submission(pool: PgPool, nuid: String, ok: bool) -> Result<(), sqlx::Error> {
    let submission_time: DateTime<Utc> = SystemTime::now().into();

    query!(
        r#"INSERT INTO submissions (nuid, ok, submission_time) VALUES ($1, $2, $3);"#,
        nuid,
        ok,
        submission_time,
    )
    .execute(&pool)
    .await?;

    Ok(())
}
