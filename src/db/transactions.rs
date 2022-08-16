use std::io::Error;

use sqlx::{PgPool, Postgres, Transaction};

pub async fn register_user_db(pool: PgPool) -> Result<(), sqlx::Error> {
    Ok(())
}
