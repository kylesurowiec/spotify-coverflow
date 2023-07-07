mod config;
mod http;
mod os;
mod server;
mod spotify;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = server::listen();

    spotify::start_auth_flow()?;
    let _ = handle.join();

    Ok(())
}
