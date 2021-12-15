use limelight::{renderer::Drawable, Renderer};
use limelight_primitives::{
    Circle, CircleLayer, Color, Hairline, HairlineLayer, Line, LineLayer, Orientation, Rect,
    RectLayer,
};
use limelight_transform::TransformUniform;
use limelight_yew::{LimelightController, LimelightComponent};
use anyhow::Result;

struct Primitives {
    layers: Vec<Box<dyn Drawable>>,
    transform: TransformUniform,
}

impl LimelightController for Primitives {
    fn draw(&mut self, renderer: &mut Renderer, _ts: f64) -> Result<limelight_yew::ShouldRequestAnimationFrame> {
        for layer in self.layers.iter_mut() {
            layer.draw(renderer)?;
        }

        Ok(false)
    }

    fn handle_drag(&mut self, x: f32, y: f32) -> limelight_yew::ShouldRequestAnimationFrame {
        self.transform.pan((x, y));
        true
    }
}

impl Default for Primitives {
    fn default() -> Self {
        let mut layers: Vec<Box<dyn Drawable>> = Vec::new();
        let transform = TransformUniform::new();

        let lines = LineLayer::new(transform.uniform());
        let rects = RectLayer::new(transform.uniform());
        let circles = CircleLayer::new(transform.uniform());
        let hairlines = HairlineLayer::new(transform.uniform());
    
        lines.buffer().set_data(vec![
            Line {
                start: [0., 0.],
                end: [0.4, 0.9],
                width: 0.03,
                color: palette::named::GOLD.into(),
            },
            Line {
                start: [-0.3, -0.3],
                end: [0.4, 0.9],
                width: 0.01,
                color: palette::named::FIREBRICK.into(),
            },
        ]);
    
        rects.buffer().set_data(vec![
            Rect {
                lower_right: [-0.3, 0.1],
                upper_left: [-0.8, 0.2],
                color: palette::named::SEAGREEN.into(),
            },
            Rect {
                lower_right: [-0.3, 0.25],
                upper_left: [-0.6, 0.35],
                color: palette::named::PALEVIOLETRED.into(),
            },
            Rect {
                lower_right: [-0.3, 0.4],
                upper_left: [-0.4, 0.5],
                color: palette::named::ORANGERED.into(),
            },
        ]);
    
        circles.buffer().set_data(vec![
            Circle {
                position: [0.3, 0.3],
                radius: 0.1,
                color: palette::named::SALMON.into(),
            },
            Circle {
                position: [-0.2, 0.3],
                radius: 0.2,
                color: Color(0x44332266),
            },
        ]);
    
        hairlines.buffer().set_data(vec![
            Hairline {
                orientation: Orientation::Horizontal,
                location: 0.65,
                color: palette::named::RED.into(),
            },
            Hairline {
                orientation: Orientation::Vertical,
                location: 0.65,
                color: palette::named::DARKBLUE.into(),
            },
            Hairline {
                orientation: Orientation::Vertical,
                location: 0.7,
                color: palette::named::DARKCYAN.into(),
            },
            Hairline {
                orientation: Orientation::Vertical,
                location: 0.75,
                color: palette::named::DARKMAGENTA.into(),
            },
        ]);

        layers.push(Box::new(lines));
        layers.push(Box::new(rects));
        layers.push(Box::new(circles));
        layers.push(Box::new(hairlines));

        Primitives {
            layers,
            transform
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<LimelightComponent<Primitives>>();
}
