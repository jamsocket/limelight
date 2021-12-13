use limelight::{renderer::Drawable, Renderer};
use limelight_primitives::{
    Circle, CircleLayer, Color, Hairline, HairlineLayer, Line, LineLayer, Orientation, Rect,
    RectLayer,
};
use limelight_transform::TransformUniform;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

fn render_primitives(gl: WebGl2RenderingContext) {
    let transform = TransformUniform::new();
    let mut lines = LineLayer::new(transform.uniform());
    let mut rects = RectLayer::new(transform.uniform());
    let mut circles = CircleLayer::new(transform.uniform());
    let mut hairlines = HairlineLayer::new(transform.uniform());

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

    let mut renderer = Renderer::new(gl);
    lines.draw(&mut renderer).unwrap();
    rects.draw(&mut renderer).unwrap();
    circles.draw(&mut renderer).unwrap();
    hairlines.draw(&mut renderer).unwrap();
}

fn get_gl() -> WebGl2RenderingContext {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap()
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    let gl = get_gl();
    render_primitives(gl);
}
