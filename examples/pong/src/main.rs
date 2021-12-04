use std::ops::Deref;

use limelight::{
    vertex_attribute, Buffer, BufferUsageHint, DrawMode, Program, Renderer,
};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, KeyboardEvent, WebGl2RenderingContext};
use yew::services::keyboard::KeyListenerHandle;
use yew::services::render::RenderTask;
use yew::services::{KeyboardService, RenderService};
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

const UPPER_BOUND: f32 = 1.;
const LOWER_BOUND: f32 = -1.;

const LEFT_BOUND: f32 = -1.;
const RIGHT_BOUND: f32 = 1.;
const PADDLE_BUFFER: f32 = 0.1;

const PADDLE_HEIGHT: f32 = 0.3;
const BALL_RADIUS: f32 = 0.05;

const PADDLE_WIDTH: f32 = 0.05;
const PADDLE_SPEED: f32 = 0.01;

const BALL_SPEED: f32 = 0.01;

struct GameState {
    paddle_position: f32,
    ball_position: [f32; 2],
    ball_direction: [f32; 2],
}

impl GameState {
    fn move_paddle(&mut self, direction: f32) {
        self.paddle_position += direction * PADDLE_SPEED;
    }

    fn step(&mut self) {
        self.ball_position = [
            self.ball_position[0] + self.ball_direction[0] * BALL_SPEED,
            self.ball_position[1] + self.ball_direction[1] * BALL_SPEED,
        ];

        if ((LEFT_BOUND + PADDLE_BUFFER - BALL_SPEED + PADDLE_WIDTH / 2.)
            ..(LEFT_BOUND + PADDLE_BUFFER + BALL_SPEED + PADDLE_WIDTH / 2.))
            .contains(&self.ball_position[0])
            && ((self.paddle_position - PADDLE_HEIGHT / 2.)
                ..(self.paddle_position + PADDLE_HEIGHT / 2.))
                .contains(&self.ball_position[1])
        {
            self.ball_direction[0] = -self.ball_direction[0];
        }

        if ((RIGHT_BOUND - PADDLE_BUFFER - BALL_SPEED - PADDLE_WIDTH / 2.)
            ..(RIGHT_BOUND - PADDLE_BUFFER + BALL_SPEED - PADDLE_WIDTH / 2.))
            .contains(&self.ball_position[0])
            && ((-self.paddle_position - PADDLE_HEIGHT / 2.)
                ..(-self.paddle_position + PADDLE_HEIGHT / 2.))
                .contains(&self.ball_position[1])
        {
            self.ball_direction[0] = -self.ball_direction[0];
        }

        if self.ball_position[1] + BALL_RADIUS > UPPER_BOUND {
            self.ball_position[1] = UPPER_BOUND - BALL_RADIUS;
            self.ball_direction[1] = -self.ball_direction[1];
        }

        if self.ball_position[1] - BALL_RADIUS < LOWER_BOUND {
            self.ball_position[1] = LOWER_BOUND + BALL_RADIUS;
            self.ball_direction[1] = -self.ball_direction[1];
        }

        // Losing Conditions
        if self.ball_position[0] > RIGHT_BOUND + BALL_RADIUS
            || self.ball_position[0] < LEFT_BOUND - BALL_RADIUS
        {
            self.ball_position = [0., 0.];
            self.ball_direction = [-self.ball_direction[0], self.ball_direction[1]];
        }
    }

    fn into_quads(&self) -> Vec<VertexDescription> {
        let left_paddle_left = LEFT_BOUND + PADDLE_BUFFER - PADDLE_WIDTH / 2.;
        let left_paddle_right = LEFT_BOUND + PADDLE_BUFFER + PADDLE_WIDTH / 2.;
        let left_paddle_top = self.paddle_position + PADDLE_HEIGHT / 2.;
        let left_paddle_bottom = self.paddle_position - PADDLE_HEIGHT / 2.;

        let right_paddle_left = RIGHT_BOUND - PADDLE_BUFFER - PADDLE_WIDTH / 2.;
        let right_paddle_right = RIGHT_BOUND - PADDLE_BUFFER + PADDLE_WIDTH / 2.;
        let right_paddle_top = -self.paddle_position + PADDLE_HEIGHT / 2.;
        let right_paddle_bottom = -self.paddle_position - PADDLE_HEIGHT / 2.;

        let ball_left = self.ball_position[0] - BALL_RADIUS / 2.;
        let ball_right = self.ball_position[0] + BALL_RADIUS / 2.;
        let ball_top = self.ball_position[1] + BALL_RADIUS / 2.;
        let ball_bottom = self.ball_position[1] - BALL_RADIUS / 2.;

        vec![
            // Left paddle upper triangle
            VertexDescription::new(left_paddle_left, left_paddle_top),
            VertexDescription::new(left_paddle_right, left_paddle_top),
            VertexDescription::new(left_paddle_left, left_paddle_bottom),
            // Left paddle lower triangle
            VertexDescription::new(left_paddle_right, left_paddle_top),
            VertexDescription::new(left_paddle_right, left_paddle_bottom),
            VertexDescription::new(left_paddle_left, left_paddle_bottom),
            // Right paddle upper triangle
            VertexDescription::new(right_paddle_left, right_paddle_top),
            VertexDescription::new(right_paddle_right, right_paddle_top),
            VertexDescription::new(right_paddle_left, right_paddle_bottom),
            // Right paddle lower triangle
            VertexDescription::new(right_paddle_right, right_paddle_top),
            VertexDescription::new(right_paddle_right, right_paddle_bottom),
            VertexDescription::new(right_paddle_left, right_paddle_bottom),
            // Ball upper triangle
            VertexDescription::new(ball_left, ball_top),
            VertexDescription::new(ball_right, ball_top),
            VertexDescription::new(ball_left, ball_bottom),
            // Ball lower triangle
            VertexDescription::new(ball_right, ball_top),
            VertexDescription::new(ball_right, ball_bottom),
            VertexDescription::new(ball_left, ball_bottom),
        ]
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            paddle_position: 0.,
            ball_direction: [1., 1.],
            ball_position: [0., 0.],
        }
    }
}

enum Msg {
    Render(f64),
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
}

struct Model {
    link: ComponentLink<Self>,
    buffer: Buffer<VertexDescription>,
    canvas_ref: NodeRef,
    renderer: Option<Renderer>,
    program: Program<VertexDescription>,
    render_handle: Option<RenderTask>,
    state: GameState,
    _key_down_handle: KeyListenerHandle,
    _key_up_handle: KeyListenerHandle,
    paddle_direction: f32,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let buffer = Buffer::new(vec![], BufferUsageHint::DynamicDraw);
        let state = GameState::default();
        let key_down_handle =
            KeyboardService::register_key_down(&yew::utils::window(), link.callback(Msg::KeyDown));
        let key_up_handle =
            KeyboardService::register_key_up(&yew::utils::window(), link.callback(Msg::KeyUp));

        let program =  Program::new(
            include_str!("../shaders/shader.frag"),
            include_str!("../shaders/shader.vert"),
            DrawMode::Triangles,
        );

        Self {
            link,
            buffer,
            canvas_ref: NodeRef::default(),
            render_handle: None,
            renderer: None,
            program,
            state,
            _key_down_handle: key_down_handle,
            _key_up_handle: key_up_handle,
            paddle_direction: 0.,
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
                self.state.move_paddle(self.paddle_direction);
                self.state.step();
                self.buffer.set_data(self.state.into_quads());

                if let Some(renderer) = self.renderer.as_mut() {
                    renderer
                        .render(&mut self.program, &self.buffer)
                        .unwrap();
                }

                self.render_handle = Some(RenderService::request_animation_frame(
                    self.link.callback(Msg::Render),
                ));
            }
            Msg::KeyDown(k) => match k.key().deref() {
                "ArrowUp" => self.paddle_direction = 1.,
                "ArrowDown" => self.paddle_direction = -1.,
                _ => (),
            },
            Msg::KeyUp(_) => {
                self.paddle_direction = 0.;
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
