use anyhow::Result;
use druid::lens::Lens;
use druid::piet::{ImageFormat, InterpolationMode};
use druid::widget::prelude::*;
use druid::{Color, Rect};

use crate::ui::UIState;

#[derive(Default)]
pub struct ImageWidget {
    url: String,
}

impl ImageWidget {
    pub fn new() -> ImageWidget {
        ImageWidget::default()
    }
}

// Our lens will apply functions that operate on a `String` to a `Container`.
impl Lens<UIState, String> for ImageWidget {
    fn with<V, F: FnOnce(&String) -> V>(&self, data: &UIState, f: F) -> V {
        f(&self.url)
    }
    fn with_mut<V, F: FnOnce(&mut String) -> V>(&self, data: &mut UIState, f: F) -> V {
        f(&mut self.url)
    }
}

pub fn download_image(url: String) -> Result<Vec<u8>> {
    Ok(reqwest::blocking::get(url.clone())?.bytes()?.to_vec())
}

impl Widget<UIState> for ImageWidget {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut UIState, _: &Env) {}

    fn update(&mut self, _: &mut UpdateCtx, _: &UIState, _: &UIState, _: &Env) {}

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &UIState, _: &Env) {}

    fn layout(&mut self, _: &mut LayoutCtx, bc: &BoxConstraints, _: &UIState, _: &Env) -> Size {
        if bc.is_width_bounded() | bc.is_height_bounded() {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        } else {
            bc.max()
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, state: &UIState, _: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();

        ctx.fill(rect, &Color::WHITE);

        // TODO: No panic
        let image_data = download_image(state.album_art).expect("Failed to download image");
        let image = ctx
            .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
            .unwrap();

        // The image is automatically scaled to fit the rect you pass to draw_image
        ctx.draw_image(
            &image,
            Rect::new(20.0, 20.0, 200.0, 200.0),
            InterpolationMode::Bilinear,
        );
    }
}
