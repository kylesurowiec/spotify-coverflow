mod http;
mod server;
mod spotify;
mod util;

#[tokio::main]
async fn main() -> std::result::Result<(), ()> {
    let config = util::get_config();
    println!("{config:#?}");

    let handle = server::listen();

    Ok(())
}
