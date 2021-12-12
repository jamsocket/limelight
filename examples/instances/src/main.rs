use limelight::{attribute, Buffer, BufferUsageHint, DrawMode, Program, Renderer};
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

#[attribute]
struct InstanceAttribute {
    instance_index: i32,
}

#[attribute]
struct VertexAttribute {
    vertex_index: i32,
}

fn render_triangle(gl: WebGl2RenderingContext) {
    let mut program = Program::new(
        include_str!("../shaders/shader.vert"),
        include_str!("../shaders/shader.frag"),
        DrawMode::Points,
    );

    let mut renderer = Renderer::new(gl);

    let instances = Buffer::new(
        (0..10)
            .map(|instance_index| InstanceAttribute { instance_index })
            .collect(),
        BufferUsageHint::StaticDraw,
    );

    let vertices = Buffer::new(
        (0..10)
            .map(|vertex_index| VertexAttribute { vertex_index })
            .collect(),
        BufferUsageHint::StaticDraw,
    );

    renderer
        .render_instanced(&mut program, &vertices, &instances)
        .unwrap();
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

    render_triangle(gl);
}
