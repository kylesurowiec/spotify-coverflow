mod config;
mod http;
mod os;
mod server;
mod spotify;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    spotify::prompt_auth_flow().await?;
    server::listen().await?;

    Ok(())
}
