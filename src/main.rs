#![feature(type_alias_impl_trait)]
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use sqlx::PgPool;

use std::error::Error;

use crate::endpoints::server::Server;
use crate::model::Model;
use std::env;

mod db;
mod endpoints;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let conn_string = match env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => format!(
            "postgres://{}:{}@127.0.0.1:{}/{}",
            "postgres", "postgres", 5432, "db"
        ),
    };

    let pool = PgPool::connect(&conn_string).await?;

    let model = Model { pool: &pool };
    let server = Server { model: &model };

    info!("Stating submission server");
    warp::serve(server.end()).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}
