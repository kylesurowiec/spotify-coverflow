mod config;
mod http;
mod os;
mod server;
mod spotify;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    ui::bootstrap();

    spotify::prompt_auth_flow()?;

    futures::future::join_all(vec![server::listen()]).await;

    Ok(())
}
