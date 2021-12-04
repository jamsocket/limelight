use limelight::{vertex_attribute, Buffer, BufferUsageHint, DrawMode, Program, Renderer, Uniform};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use yew::services::render::RenderTask;
use yew::services::RenderService;
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

struct Animation {
    program: Program<VertexDescription>,
    buffer: Buffer<VertexDescription>,
    uniform: Uniform<[f32; 3]>,
}

impl Animation {
    pub fn new() -> Self {
        let buffer = Buffer::new(vec![], BufferUsageHint::DynamicDraw);
        let uniform = Uniform::new([0., 0., 0.]);

        let program = Program::new(
            include_str!("../shaders/shader.frag"),
            include_str!("../shaders/shader.vert"),
            DrawMode::Triangles,
        )
        .with_uniform("u_color", uniform.clone());

        Animation {
            buffer,
            program,
            uniform,
        }
    }

    pub fn render(&mut self, time: f64, renderer: &mut Renderer) {
        let theta1 = time as f32 / 1000.;
        let theta2 = theta1 + (std::f32::consts::TAU / 3.);
        let theta3 = theta2 + (std::f32::consts::TAU / 3.);

        self.buffer.set_data(vec![
            VertexDescription::new(theta1.cos(), theta1.sin()),
            VertexDescription::new(theta2.cos(), theta2.sin()),
            VertexDescription::new(theta3.cos(), theta3.sin()),
        ]);

        let r = (time as f32 / 3000.).sin() / 2. + 0.5;
        let g = (time as f32 / 5000.).sin() / 2. + 0.5;
        let b = (time as f32 / 7000.).sin() / 2. + 0.5;

        self.uniform.set_value([r, g, b]);

        renderer.render(&mut self.program, &self.buffer).unwrap();
    }
}

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
    canvas_ref: NodeRef,
    renderer: Option<Renderer>,
    animation: Animation,
    render_handle: Option<RenderTask>,
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
            animation: Animation::new(),
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

        self.renderer = Some(Renderer::new(gl));

        self.render_handle = Some(RenderService::request_animation_frame(
            self.link.callback(Msg::Render),
        ));
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(ts) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    self.animation.render(ts, renderer);
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
