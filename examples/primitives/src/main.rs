use limelight::{Renderer, renderer::Drawable};
use limelight_primitives::{LineLayer, Line};
use wasm_bindgen::JsCast;
use limelight_transform::TransformUniform;
use web_sys::WebGl2RenderingContext;

fn render_lines(gl: WebGl2RenderingContext) {
    let transform = TransformUniform::new();
    let mut line_layer = LineLayer::new(transform.uniform());
    let mut renderer = Renderer::new(gl);

    let lines = line_layer.buffer();
    lines.set_data(vec![
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

    line_layer.draw(&mut renderer).unwrap();
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

    render_lines(gl);
}
