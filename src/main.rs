use std::error::Error;

mod endpoints;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    warp::serve(server::end()).run(([0, 0, 0, 0], 8000)).await;
    Ok(())
}
