use druid::widget::Label;
use druid::{AppLauncher, WindowDesc};
use tokio::task::JoinHandle;
use tokio::time;

const TICKRATE: time::Duration = time::Duration::from_secs(1);

pub fn tick(_event_sink: druid::ExtEventSink) -> JoinHandle<()> {
    tokio::spawn(async move {
        let tick = time::sleep(TICKRATE);
        tokio::pin!(tick);

        loop {
            tokio::select! {
                () = &mut tick => {
                    println!("TICK");
                    tick.as_mut().reset(time::Instant::now() + TICKRATE);
                    // let config = crate::config::get().expect("Failed to read config");
                    // let _token = crate::spotify::get_new_token(&config.oauth_refresh_token).await;
                }
            }
        }
    })
}

pub fn bootstrap() {
    let main_window = WindowDesc::new(create_song_label())
        .window_size((600.0, 400.0))
        .title("My first Druid App");

    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();

    tick(event_sink);

    launcher
        .log_to_console()
        .launch("".to_string())
        .expect("Failed to launch spotify-coverflow")
}

pub fn create_song_label() -> druid::widget::Label<String> {
    Label::new("Testing 123")
}
