use limelight::{DrawMode, DummyBuffer, Program, Renderer, vertex_attribute};
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext;

#[vertex_attribute]
struct InstanceAttribute {
    index: u8,
}



fn render_triangle(gl: WebGl2RenderingContext) {
    // limelight doesn't touch the DOM at all. Use your preferred
    // framework to create a canvas and create a WebGL2 context
    // from it.

    // Create a shader program by passing in GLSL code as strings for
    // the fragment and vertex shaders.
    let mut program = Program::new(
        include_str!("../shaders/shader.frag"),
        include_str!("../shaders/shader.vert"),
        DrawMode::Points,
    );

    // Create a renderer. The renderer becomes the owner of the
    // WebGl2RenderingContext, to ensure that its internal representation
    // of the GPU state is always accureate.
    let mut renderer = Renderer::new(gl);

    // Run the program, rendering the results to the screen. We are
    // not passing any vertex attribute data, so we use a `DummyBuffer`
    // which renders three vertices: one for each corner of a triangle.
    renderer.render_instanced(&mut program, &DummyBuffer::new(10), 1).unwrap();
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
