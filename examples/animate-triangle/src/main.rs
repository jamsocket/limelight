use std::rc::Rc;

use gl_layers::buffer::{AttributeBuffer, BufferUsageHint};
use gl_layers::draw_modes::DrawMode;
use gl_layers::plan::{Renderer, Stage};
use gl_layers::program::Program;
use gl_layers::state::State;
use gl_layers::vertex_attribute;
use gl_layers::vertex_attribute::VertexAttribute;
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
    buffer: Rc<AttributeBuffer<VertexDescription>>,
    canvas_ref: NodeRef,
    renderer: Renderer,
    gl: Option<WebGl2RenderingContext>,
    render_handle: Option<RenderTask>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let buffer = AttributeBuffer::new(BufferUsageHint::StaticDraw);
        let program = Program::new(
            include_str!("../shaders/shader.frag"),
            include_str!("../shaders/shader.vert"),
        );
        let renderer = Renderer::new(vec![Stage::new(
            program,
            buffer.clone(),
            State::default(),
            DrawMode::Triangles,
        )]);

        Self {
            link,
            buffer,
            canvas_ref: NodeRef::default(),
            renderer,
            gl: None,
            render_handle: None,
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

        self.gl = Some(gl);
        self.render_handle = Some(RenderService::request_animation_frame(
            self.link.callback(Msg::Render),
        ));
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(ts) => {
                let ts = ts / 1000.;
                self.buffer.set_data(vec![
                    VertexDescription::new(-0.5 * ts.cos() as f32, -0.5 * ts.sin() as f32),
                    VertexDescription::new(0.5 * (ts/3.).cos() as f32, -0.5 * (ts/5.).cos() as f32),
                    VertexDescription::new(0.5, 0.5),        
                ]);

                if let Some(gl) = self.gl.as_ref() {
                    self.renderer.render(gl);
                }

                self.render_handle = Some(RenderService::request_animation_frame(
                    self.link.callback(Msg::Render)
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
