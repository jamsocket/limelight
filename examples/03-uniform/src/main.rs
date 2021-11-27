use limelight::{DrawMode, DummyBuffer, Program, Renderer, Uniform};
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

fn render_triangles_with_uniform(gl: WebGl2RenderingContext) {
    let rotate_uniform = Uniform::new(std::f32::consts::PI / 3.4);
    let scale_uniform = Uniform::new([0.5, 0.8]);
    let color_uniform = Uniform::new([0.9, 0.2, 0.3]);

    let program = Program::new(
        include_str!("../shaders/shader.frag"),
        include_str!("../shaders/shader.vert"),
        DrawMode::Triangles,
    )
    .with_uniform("u_rotate", rotate_uniform)
    .with_uniform("u_scale", scale_uniform)
    .with_uniform("u_color", color_uniform)
    .gpu_init(&gl)
    .unwrap();

    let renderer = Renderer::new(gl);
    renderer.render(&program, &DummyBuffer::new(3)).unwrap();
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
    let gl = get_gl();

    render_triangles_with_uniform(gl);
}
