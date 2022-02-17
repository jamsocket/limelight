use anyhow::Result;
use limelight::{DrawMode, DummyBuffer, Program, Renderer, Uniform};
use limelight_yew::{LimelightComponent, LimelightController, ShouldRequestAnimationFrame};

struct Model {
    program: Program<(), ()>,
    time_uniform: Uniform<f32>,
    pos_uniform: Uniform<[f32; 2]>,
}

impl Default for Model {
    fn default() -> Model {
        let time_uniform = Uniform::new(0.0);
        let pos_uniform = Uniform::new([0.0, 0.0]);

        let program = Program::new(
            include_str!("../shaders/shader.vert"),
            include_str!("../shaders/shader.frag"),
            DrawMode::Triangles,
        )
        .with_uniform("u_time", time_uniform.clone())
        .with_uniform("u_pos", pos_uniform.clone());

        Self {
            program,
            time_uniform,
            pos_uniform,
        }
    }
}

impl LimelightController for Model {
    fn draw(&mut self, renderer: &mut Renderer, ts: f64) -> Result<ShouldRequestAnimationFrame> {
        let ts = ts / 1000.;
        self.time_uniform.set_value(ts as _);

        renderer
            .render(&mut self.program, &DummyBuffer::new(3))
            .unwrap();

        Ok(true)
    }

    fn handle_mousemove(&mut self, x: f32, y: f32) -> ShouldRequestAnimationFrame {
        self.pos_uniform.set_value([x, y]);

        true
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::start_app::<LimelightComponent<Model>>();
}
