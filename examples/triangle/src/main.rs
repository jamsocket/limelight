use gl_layers::buffer::{AttributeBuffer, BufferUsageHint};
use gl_layers::draw_modes::DrawMode;
use gl_layers::plan::{Renderer, Stage};
use gl_layers::program::Program;
use gl_layers::state::State;
use gl_layers::vertex_attribute::VertexAttribute;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;
use gl_layers::vertex_attribute;

#[vertex_attribute]
struct VertexDescription {
    position: [f32; 2],
}

impl VertexDescription {
    pub fn new(x: f32, y: f32) -> Self {
        VertexDescription { position: [x, y] }
    }
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

    let buffer = AttributeBuffer::new(BufferUsageHint::StaticDraw);
    let program = Program::new(
        include_str!("../shaders/shader.frag"),
        include_str!("../shaders/shader.vert"),
    );
    let renderer = Renderer::new(vec![
        Stage::new(program, buffer.clone(), State::default(), DrawMode::Triangles),
    ]);

    buffer.set_data(vec![
        VertexDescription::new(-0.5, -0.5),
        VertexDescription::new(0.5, -0.5),
        VertexDescription::new(0.5, 0.5),
    ]);

    let gl = get_gl();
    renderer.render(&gl);
}
