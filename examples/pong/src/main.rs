use anyhow::Result;
use limelight::{attribute, Buffer, BufferUsageHint, DrawMode, Program, Renderer};
use limelight_yew::{LimelightComponent, LimelightController, ShouldRequestAnimationFrame, KeyCode, ShouldCancelEvent};

#[attribute]
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

    fn as_quads(&self) -> Vec<VertexDescription> {
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

struct PongGame {
    buffer: Buffer<VertexDescription>,
    program: Program<VertexDescription, ()>,
    state: GameState,
    paddle_direction: f32,
}

impl LimelightController for PongGame {
    fn draw(&mut self, renderer: &mut Renderer, _: f64) -> Result<ShouldRequestAnimationFrame> {
        self.state.move_paddle(self.paddle_direction);
        self.state.step();
        self.buffer.set_data(self.state.as_quads());

        renderer.render(&mut self.program, &self.buffer).unwrap();

        Ok(true)
    }

    fn handle_key_down(&mut self, key: KeyCode) -> (ShouldRequestAnimationFrame, ShouldCancelEvent) {
        match key {
            KeyCode::ArrowUp => self.paddle_direction = -1.,
            KeyCode::ArrowDown => self.paddle_direction = 1.,
            _ => return (false, false)
        }

        (false, true)
    }

    fn handle_key_up(&mut self, _: KeyCode) -> (ShouldRequestAnimationFrame, ShouldCancelEvent) {
        self.paddle_direction = 0.;
        
        (false, true)
    }
}

impl Default for PongGame {
    fn default() -> Self {
        let buffer = Buffer::new(vec![], BufferUsageHint::DynamicDraw);
        let state = GameState::default();

        let program = Program::new(
            include_str!("../shaders/shader.vert"),
            include_str!("../shaders/shader.frag"),
            DrawMode::Triangles,
        );

        Self {
            buffer,
            program,
            state,
            paddle_direction: 0.,
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<LimelightComponent<PongGame>>();
}
