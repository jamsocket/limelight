use std::rc::Rc;
use limelight::{AttributeBuffer, BufferUsageHint, DrawMode, GlProgram, Program, Renderer, Uniform, vertex_attribute};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use yew::services::render::RenderTask;
use yew::services::RenderService;
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

#[vertex_attribute]
struct VertexDescription {
    position: [f32; 2],
}

impl VertexDescription {
    pub fn new(x: f32, y: f32) -> Self {
        VertexDescription { position: [x, y] }
    }
}

enum Msg {
    Render(f64),
}

struct Model {
    link: ComponentLink<Self>,
    buffer: AttributeBuffer<VertexDescription>,
    canvas_ref: NodeRef,
    renderer: Option<Renderer>,
    program: Option<GlProgram<VertexDescription>>,
    render_handle: Option<RenderTask>,
    uniform: Rc<Uniform<f32>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut buffer = AttributeBuffer::new(BufferUsageHint::StaticDraw);
        buffer.set_data(vec![
            VertexDescription::new(0.5, 0.5),
            VertexDescription::new(0.5, -0.5),
            VertexDescription::new(-0.5, -0.5),
        ]);

        Self {
            link,
            buffer,
            canvas_ref: NodeRef::default(),
            render_handle: None,
            renderer: None,
            program: None,
            uniform: Uniform::new(0.0),
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: WebGl2RenderingContext = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        self.program = Some(
            Program::new(
                include_str!("../shaders/shader.frag"),
                include_str!("../shaders/shader.vert"),
                DrawMode::Triangles,
            )
            .with_uniform("u_time", self.uniform.clone())
            .gpu_init(&gl)
            .unwrap(),
        );

        self.renderer = Some(Renderer::new(gl));

        self.render_handle = Some(RenderService::request_animation_frame(
            self.link.callback(Msg::Render),
        ));
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(ts) => {
                let ts = ts / 1000.;
                self.uniform.set_value(ts as _);

                if let Some(renderer) = self.renderer.as_ref() {
                    renderer
                        .render(self.program.as_ref().unwrap(), &self.buffer)
                        .unwrap();
                }

                self.render_handle = Some(RenderService::request_animation_frame(
                    self.link.callback(Msg::Render),
                ));
            }
        }

        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas
                height="900"
                width="900"
                ref={self.canvas_ref.clone()} />
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<Model>();
}
