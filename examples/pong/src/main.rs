use anyhow::Result;
use limelight::renderer::Drawable;
use limelight::Renderer;
use limelight_primitives::{Circle, CircleLayer, Rect, RectLayer};
use limelight_yew::{
    KeyCode, LimelightComponent, LimelightController, ShouldCancelEvent,
    ShouldRequestAnimationFrame,
};

const UPPER_BOUND: f32 = 1.;
const LOWER_BOUND: f32 = -1.;

const LEFT_BOUND: f32 = -1.;
const RIGHT_BOUND: f32 = 1.;
const PADDLE_BUFFER: f32 = 0.1;

const PADDLE_HEIGHT: f32 = 0.3;
const BALL_RADIUS: f32 = 0.025;

const PADDLE_WIDTH: f32 = 0.05;
const PADDLE_SPEED: f32 = 0.01;

const BALL_SPEED: f32 = 0.01;

struct GameState {
    paddle_position: f32,
    ball_position: [f32; 2],
    ball_direction: [f32; 2],
    last_time: f64,
}

impl GameState {
    fn step(&mut self, ts: f64, paddle_direction: f32) {
        let time_delta = ((ts - self.last_time) / 20.) as f32;
        self.last_time = ts;

        self.paddle_position += paddle_direction * PADDLE_SPEED * time_delta;
        self.ball_position = [
            self.ball_position[0] + self.ball_direction[0] * BALL_SPEED * time_delta,
            self.ball_position[1] + self.ball_direction[1] * BALL_SPEED * time_delta,
        ];

        if ((LEFT_BOUND + PADDLE_BUFFER - BALL_SPEED + PADDLE_WIDTH / 2.)
            ..=(LEFT_BOUND + PADDLE_BUFFER + BALL_SPEED + PADDLE_WIDTH / 2.))
            .contains(&self.ball_position[0])
            && ((self.paddle_position - PADDLE_HEIGHT / 2.)
                ..(self.paddle_position + PADDLE_HEIGHT / 2.))
                .contains(&self.ball_position[1])
        {
            self.ball_direction[0] = -self.ball_direction[0];
        }

        if ((RIGHT_BOUND - PADDLE_BUFFER - BALL_SPEED - PADDLE_WIDTH / 2.)
            ..=(RIGHT_BOUND - PADDLE_BUFFER + BALL_SPEED - PADDLE_WIDTH / 2.))
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
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            paddle_position: 0.,
            ball_direction: [1., 1.],
            ball_position: [0., 0.],
            last_time: 0.,
        }
    }
}

struct PongGame {
    paddles: RectLayer,
    ball: CircleLayer,
    state: GameState,
    paddle_direction: f32,
}

impl LimelightController for PongGame {
    fn draw(&mut self, renderer: &mut Renderer, ts: f64) -> Result<ShouldRequestAnimationFrame> {
        self.state.step(ts, self.paddle_direction);

        let left_paddle_left = LEFT_BOUND + PADDLE_BUFFER - PADDLE_WIDTH / 2.;
        let left_paddle_right = LEFT_BOUND + PADDLE_BUFFER + PADDLE_WIDTH / 2.;
        let left_paddle_top = self.state.paddle_position + PADDLE_HEIGHT / 2.;
        let left_paddle_bottom = self.state.paddle_position - PADDLE_HEIGHT / 2.;

        let right_paddle_left = RIGHT_BOUND - PADDLE_BUFFER - PADDLE_WIDTH / 2.;
        let right_paddle_right = RIGHT_BOUND - PADDLE_BUFFER + PADDLE_WIDTH / 2.;
        let right_paddle_top = -self.state.paddle_position + PADDLE_HEIGHT / 2.;
        let right_paddle_bottom = -self.state.paddle_position - PADDLE_HEIGHT / 2.;

        self.paddles.buffer().set_data(vec![
            Rect {
                upper_left: [left_paddle_left, left_paddle_top],
                lower_right: [left_paddle_right, left_paddle_bottom],
                color: palette::named::ORANGERED.into(),
            },
            Rect {
                upper_left: [right_paddle_left, right_paddle_top],
                lower_right: [right_paddle_right, right_paddle_bottom],
                color: palette::named::ORANGERED.into(),
            },
        ]);

        self.ball.buffer().set_data(vec![Circle {
            position: self.state.ball_position,
            radius: BALL_RADIUS,
            color: palette::named::NAVY.into(),
        }]);

        self.paddles.draw(renderer)?;
        self.ball.draw(renderer)?;

        Ok(true)
    }

    fn handle_key_down(
        &mut self,
        key: KeyCode,
    ) -> (ShouldRequestAnimationFrame, ShouldCancelEvent) {
        match key {
            KeyCode::ArrowUp => self.paddle_direction = -1.,
            KeyCode::ArrowDown => self.paddle_direction = 1.,
            _ => return (false, false),
        }

        (false, true)
    }

    fn handle_key_up(&mut self, key: KeyCode) -> (ShouldRequestAnimationFrame, ShouldCancelEvent) {
        match key {
            KeyCode::ArrowUp | KeyCode::ArrowDown => self.paddle_direction = 0.,
            _ => return (false, false),
        }

        (false, true)
    }
}

impl Default for PongGame {
    fn default() -> Self {
        Self {
            state: GameState::default(),
            paddle_direction: 0.,
            ball: CircleLayer::new(),
            paddles: RectLayer::new(),
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<LimelightComponent<PongGame>>();
}
