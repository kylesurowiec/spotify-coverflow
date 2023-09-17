mod widgets;

use anyhow::Result;
use druid::widget::{Flex, Label, MainAxisAlignment};
use druid::{AppLauncher, Color, Data, Lens, Widget, WidgetExt, WindowDesc};
use tokio::task::JoinHandle;
use tokio::time;

use self::widgets::image::ImageWidget;
use crate::spotify;

const TICKRATE: time::Duration = time::Duration::from_secs(3);

#[derive(Clone, Default, Lens, Data)]
pub struct UIState {
    song: String,
    artist: String,
    album: String,
    album_art: String,
}

impl UIState {
    pub fn new(song: String, artist: String, album: String, album_art: String) -> Self {
        UIState {
            song,
            artist,
            album,
            album_art,
        }
    }
}

pub fn bootstrap() -> Result<()> {
    let main_window = WindowDesc::new(current_song_labels())
        .show_titlebar(false)
        .window_size((1200.0, 1000.0))
        .title("Spotify Coverflow");
    let launcher = AppLauncher::with_window(main_window);
    let event_sink = launcher.get_external_handle();

    tick(event_sink)?;

    launcher
        .log_to_console()
        .launch(UIState::default())
        .expect("Failed to launch spotify-coverflow");

    Ok(())
}

pub fn tick(event_sink: druid::ExtEventSink) -> Result<JoinHandle<()>> {
    Ok(tokio::spawn(async move {
        let tick = time::sleep(TICKRATE);
        tokio::pin!(tick);

        loop {
            tokio::select! {
                () = &mut tick => {
                    println!("TICK");
                    let current_song = spotify::get_current_song().await;

                    if let Ok(current_song) = current_song {
                        event_sink.add_idle_callback(move |data: &mut UIState| {
                            *data = UIState::new(
                                current_song.item.name,
                                current_song.item.artists[0].name.clone(),
                                current_song.item.album.name,
                                current_song.item.album.images[0].url.clone(),
                            );
                        });
                    }

                    tick.as_mut().reset(time::Instant::now() + TICKRATE);
                }
            }
        }
    }))
}

pub fn current_song_labels() -> impl Widget<UIState> {
    let song_label = Label::new(|data: &String, _: &_| format!("Song: {data}"))
        .expand_width()
        .lens(UIState::song);

    let artist_label = Label::new(|data: &String, _: &_| format!("Artist: {data}"))
        .expand_width()
        .lens(UIState::artist);

    let album_label = Label::new(|data: &String, _: &_| format!("Album: {data}"))
        .expand_width()
        .lens(UIState::album);

    let album_art = ImageWidget::new().expand_width().lens(UIState::album_art);

    Flex::column()
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_default_spacer()
        .with_flex_child(song_label, 1.0)
        .with_flex_child(artist_label, 1.0)
        .with_flex_child(album_label, 1.0)
        .with_flex_child(album_art, 1.0)
        .align_right()
        .border(Color::RED, 10.0)
}
