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
    challenge_string: String,
    solution: HashMap<String, u64>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    // Insert the applicant
    let registration_time: DateTime<Utc> = SystemTime::now().into();

    query!(
        r#"INSERT INTO applicants (nuid, applicant_name, registration_time, token)
         VALUES ($1, $2, $3, $4);"#,
        nuid,
        name,
        registration_time,
        token
    )
    .execute(&mut tx)
    .await?;

    let ser_solution = match serde_json::to_value(&solution) {
        Ok(val) => val,
        Err(_) => todo!("Figure out how to handle the serde error properly"),
    };

    query!(
        r#"INSERT INTO solutions (challenge_string, solution, token) VALUES
       ($1, $2, $3);"#,
        challenge_string,
        ser_solution,
        token,
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn retreive_token_db(pool: &PgPool, nuid: String) -> Result<Uuid, sqlx::Error> {
    let record = query!(r#"SELECT token FROM applicants WHERE nuid=$1"#, nuid)
        .fetch_one(pool)
        .await?;

    Ok(record.token)
}

pub async fn retreive_soln(
    pool: &PgPool,
    token: Uuid,
) -> Result<(HashMap<String, u64>, i32), sqlx::Error> {
    let record = query!(
        r#"SELECT solution_id, solution FROM solutions WHERE token=$1"#,
        token
    )
    .fetch_one(pool)
    .await?;

    match serde_json::from_value(record.solution) {
        Ok(soln) => Ok((soln, record.solution_id)),
        Err(_e) => panic!("solution didn't deserialize properly"),
    }
}

pub async fn write_submission(
    pool: PgPool,
    solution_id: i32,
    token: Uuid,
    ok: bool,
) -> Result<(), sqlx::Error> {
    let submission_time: DateTime<Utc> = SystemTime::now().into();

    query!(
        r#"INSERT INTO submissions (solution_id, token, ok, submission_time) VALUES ($1, $2, $3, $4);"#,
        solution_id,
        token,
        ok,
        submission_time,
    ).execute(&pool).await?;

    Ok(())
}
