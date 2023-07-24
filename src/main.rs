mod config;
mod http;
mod os;
mod server;
mod spotify;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    spotify::prompt_auth_flow()?;

    let server_thread = server::listen();
    let ui_thread = ui::bootstrap();

    futures::future::join_all(vec![server_thread, ui_thread]).await;

    Ok(())
}
