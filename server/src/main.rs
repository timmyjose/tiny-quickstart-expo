use log::info;
use server::create_server;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    env_logger::init();

    let server = create_server()?;
    info!("Starting server on port 8080");
    Ok(server.await?)
}
