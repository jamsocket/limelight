use limelight::{DrawMode, DummyBuffer, Program, Renderer};
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

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
    let gl = get_gl();

    let program = Program::new(
        include_str!("../shaders/shader.frag"),
        include_str!("../shaders/shader.vert"),
        DrawMode::Triangles
    ).gpu_init(&gl).unwrap();

    let renderer = Renderer::new(gl);
    renderer.render(&program, &DummyBuffer::new(3)).unwrap();
}
