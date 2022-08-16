extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use sqlx::PgPool;

use std::error::Error;

use crate::endpoints::server;
use crate::model::Model;

mod db;
mod endpoints;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let conn_string = format!(
        "postgres://{}:{}@127.0.0.1:{}/{}",
        "postgres", "postgres", 5432, "db"
    );
    let pool = PgPool::connect(&conn_string).await?;

    let model = Model { pool };

    info!("Stating submission server");
    warp::serve(server::end()).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}
