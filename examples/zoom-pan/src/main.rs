use limelight::{attribute, Buffer, BufferUsageHint, DrawMode, Program, Renderer};
use limelight_transform::TransformUniform;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, MouseEvent, WebGl2RenderingContext, WheelEvent};
use yew::services::render::RenderTask;
use yew::services::RenderService;
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

const HEIGHT: i32 = 900;
const WIDTH: i32 = 900;

struct Scene {
    program: Program<VertexDescription>,
    buffer: Buffer<VertexDescription>,
    transform: TransformUniform,
}

impl Scene {
    pub fn new() -> Self {
        let theta1 = 0.;
        let theta2 = theta1 + (std::f32::consts::TAU / 3.);
        let theta3 = theta2 + (std::f32::consts::TAU / 3.);

        let data = vec![
            VertexDescription::new(theta1.cos(), theta1.sin()),
            VertexDescription::new(theta2.cos(), theta2.sin()),
            VertexDescription::new(theta3.cos(), theta3.sin()),
        ];

        let buffer = Buffer::new(data, BufferUsageHint::DynamicDraw);
        let transform = TransformUniform::new();

        let program = Program::new(
            include_str!("../shaders/shader.frag"),
            include_str!("../shaders/shader.vert"),
            DrawMode::Triangles,
        )
        .with_uniform("u_transform", transform.uniform());

        Scene {
            buffer,
            program,
            transform,
        }
    }

    pub fn pan(&mut self, x: f32, y: f32) {
        self.transform.pan((x, y));
    }

    pub fn scale(&mut self, scale_factor: f32, scale_center: (f32, f32)) {
        self.transform.scale(scale_factor, scale_center);
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
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
    MouseMove(MouseEvent),
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseWheel(WheelEvent),
}

struct Model {
    link: ComponentLink<Self>,
    canvas_ref: NodeRef,
    renderer: Option<Renderer>,
    scene: Scene,
    render_handle: Option<RenderTask>,
    drag_origin: Option<(i32, i32)>,
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
            scene: Scene::new(),
            drag_origin: None,
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
            Msg::Render(_) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    self.scene.render(renderer);
                }

                self.render_handle = Some(RenderService::request_animation_frame(
                    self.link.callback(Msg::Render),
                ));
            }
            Msg::MouseDown(e) => {
                self.drag_origin = Some((e.offset_x(), e.offset_y()));
            }
            Msg::MouseUp(_) => {
                self.drag_origin = None;
            }
            Msg::MouseMove(e) => {
                if let Some((origin_x, origin_y)) = self.drag_origin {
                    let (new_x, new_y) = (e.offset_x(), e.offset_y());

                    self.scene.pan(
                        2. * (new_x - origin_x) as f32 / WIDTH as f32,
                        2. * -(new_y - origin_y) as f32 / HEIGHT as f32,
                    );

                    self.drag_origin = Some((new_x, new_y));
                }
            }
            Msg::MouseWheel(e) => {
                let scroll_amount = e.delta_y();
                let scale_factor = 1. + scroll_amount as f32 / 100.;

                let pin_x = (2 * e.offset_x()) as f32 / WIDTH as f32 - 1.;
                let pin_y = -((2 * e.offset_y()) as f32 / HEIGHT as f32 - 1.);

                self.scene.scale(scale_factor, (pin_x, pin_y));

                e.prevent_default();
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
                style="background: #eee;"
                height={HEIGHT.to_string()}
                width={WIDTH.to_string()}
                onmousedown={self.link.callback(Msg::MouseDown)}
                onmousemove={self.link.callback(Msg::MouseMove)}
                onmouseup={self.link.callback(Msg::MouseUp)}
                onwheel={self.link.callback(Msg::MouseWheel)}
                ref={self.canvas_ref.clone()} />
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<Model>();
}
