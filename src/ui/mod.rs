use druid::widget::Label;
use druid::{AppLauncher, Widget, WindowDesc};
use tokio::task::JoinHandle;

pub fn bootstrap() -> JoinHandle<()> {
    tokio::spawn(async move {
        let config = crate::config::get().expect("Failed to read config");
        let token = crate::spotify::get_new_token(&config.oauth_refresh_token).await;

        init_ui();
    })
}

fn init_ui() {
    let main_window = WindowDesc::new(Label::new("Testing 123"))
        .window_size((600.0, 400.0))
        .title("My first Druid App");
    let initial_data = ();

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
