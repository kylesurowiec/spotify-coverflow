mod config;
mod http;
mod os;
mod server;
mod spotify;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    match config::get()?.refresh_token {
        | Some(refresh_token) => {
            let res = spotify::auth::get_new_token(&refresh_token).await?;
            config::update_token(res.access_token)?;
        },
        | None => {
            spotify::auth::prompt_auth_flow()?;
        },
    };

    ui::bootstrap()?;

    futures::future::join_all(vec![server::listen()]).await;

    Ok(())
}
