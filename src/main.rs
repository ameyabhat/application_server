#![feature(type_alias_impl_trait)]
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use sqlx::PgPool;

use std::error::Error;

use std::env;

mod db;
mod endpoints;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let conn_string = match env::var("DAT&ABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => format!(
            "postgres://{}:{}@127.0.0.1:{}/{}",
            "postgres", "postgres", 5432, "db"
        ),
    };

    let pool = PgPool::connect(&conn_string).await.ok();

    info!("Stating submission server");
    warp::serve(endpoints::end(pool))
        .run(([0, 0, 0, 0], 8000))
        .await;
    Ok(())
}
