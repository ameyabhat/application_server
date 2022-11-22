extern crate pretty_env_logger;
#[macro_use]
extern crate log;
use sqlx::PgPool;

use std::error::Error;

use crate::config::get_configuration;

mod config;
mod db;
mod endpoints;
mod model;

// Gonna need to handle TLS certs here when I deploy - lets look at NGINX
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let configuration = get_configuration().expect("Failed to read configuration file");

    let conn_string = configuration.connection_string();

    let pool = PgPool::connect(&conn_string).await;

    let o = match pool {
        Ok(p) => {
            info!("Connection established to Postgres DB");
            Some(p)
        }
        Err(e) => {
            panic!("Error connecting to db: {}", e);
        }
    };

    info!("Starting submission server");

    warp::serve(endpoints::end(o))
        .run(([0, 0, 0, 0], configuration.port()))
        .await;

    Ok(())
}
