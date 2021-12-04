use limelight::{vertex_attribute, Buffer, DrawMode, Program, Renderer, BufferUsageHint};
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

// This attribute macro derives a number of traits, including `VertexAttribute`, which
// is required for a type to be used in an `AttributeBuffer`.
#[vertex_attribute]
struct VertexDescription {
    position: [f32; 2], // field names are mapped to variables in the shader.
}

impl VertexDescription {
    pub fn new(x: f32, y: f32) -> Self {
        VertexDescription { position: [x, y] }
    }
}

fn render_triangles(gl: WebGl2RenderingContext) {
    let mut program = Program::new(
        include_str!("../shaders/shader.frag"),
        include_str!("../shaders/shader.vert"),
        DrawMode::Triangles,
    );

    let mut renderer = Renderer::new(gl);
    
    let data = vec![
        // Lower-left triangle.
        VertexDescription::new(-0.1, -0.1),
        VertexDescription::new(-0.5, -0.1),
        VertexDescription::new(-0.5, -0.5),
        // Upper-right triangle.
        VertexDescription::new(0.1, 0.1),
        VertexDescription::new(0.5, 0.1),
        VertexDescription::new(0.5, 0.5),
    ];

    let buffer: Buffer<VertexDescription> =
        Buffer::new(data, BufferUsageHint::StaticDraw);

    renderer.render(&mut program, &buffer).unwrap();
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

    render_triangles(gl);
}
