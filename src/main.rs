mod http;
mod os;
mod server;
mod spotify;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = server::listen();

    spotify::start_auth_flow()?;

    let _ = handle.join();

    Ok(())
}
