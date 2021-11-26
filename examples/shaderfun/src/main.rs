use std::rc::Rc;

use gl_layers::buffer::{DummyBuffer};
use gl_layers::draw_modes::DrawMode;
use gl_layers::gpu_init::GpuInit;
use gl_layers::program::{GlProgram, Program};
use gl_layers::renderer::Renderer;
use gl_layers::uniform::Uniform;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, WebGl2RenderingContext};
use yew::services::render::RenderTask;
use yew::services::RenderService;
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

enum Msg {
    Render(f64),
    MouseMove(MouseEvent),
}

struct Model {
    link: ComponentLink<Self>,
    canvas_ref: NodeRef,
    renderer: Option<Renderer>,
    program: Option<GlProgram<()>>,
    render_handle: Option<RenderTask>,
    time_uniform: Rc<Uniform<f32>>,
    pos_uniform: Rc<Uniform<[f32; 2]>>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            canvas_ref: NodeRef::default(),
            render_handle: None,
            renderer: None,
            program: None,
            time_uniform: Uniform::new(0.0),
            pos_uniform: Uniform::new([0.0, 0.0]),
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
            .with_uniform("u_time", self.time_uniform.clone())
            .with_uniform("u_pos", self.pos_uniform.clone())
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
                self.time_uniform.set_value(ts as _);

                if let Some(renderer) = self.renderer.as_ref() {
                    renderer
                        .render(self.program.as_ref().unwrap(), &DummyBuffer::new(3))
                        .unwrap();
                }

                self.render_handle = Some(RenderService::request_animation_frame(
                    self.link.callback(Msg::Render),
                ));
            },
            Msg::MouseMove(e) => {
                let x = e.offset_x() as f32 / 450. - 1.;
                let y = -e.offset_y() as f32 / 450. + 1.;
                self.pos_uniform.set_value([x, y]);
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
                onmousemove={self.link.callback(Msg::MouseMove)}
                ref={self.canvas_ref.clone()} />
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<Model>();
}
