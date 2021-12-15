use anyhow::Result;
use limelight::{renderer::Drawable, Renderer};
use limelight_primitives::{Circle, CircleLayer, Rect, RectLayer};
use limelight_yew::{LimelightComponent, LimelightController};

struct Primitives {
    rects: RectLayer,
    circles: CircleLayer,
}

impl LimelightController for Primitives {
    fn draw(
        &mut self,
        renderer: &mut Renderer,
        _ts: f64,
    ) -> Result<limelight_yew::ShouldRequestAnimationFrame> {
        self.rects.draw(renderer)?;
        self.circles.draw(renderer)?;

        Ok(false)
    }
}

impl Default for Primitives {
    fn default() -> Self {
        let rects = RectLayer::new();
        let circles = CircleLayer::new();

        rects.buffer().set_data(vec![
            Rect {
                lower_right: [0.4, 0.1],
                upper_left: [-0.8, 0.2],
                color: palette::named::TOMATO.into(),
            },
            Rect {
                lower_right: [0.4, 0.25],
                upper_left: [-0.6, 0.5],
                color: palette::named::SLATEBLUE.into(),
            },
        ]);

        circles.buffer().set_data(vec![
            Circle {
                position: [0., 0.25],
                radius: 0.2,
                color: palette::named::WHITE.into(),
            },
            Circle {
                position: [0., 0.25],
                radius: 0.1,
                color: palette::named::ORANGERED.into(),
            },
        ]);

        Primitives { rects, circles }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<LimelightComponent<Primitives>>();
}
