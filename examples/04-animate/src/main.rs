use anyhow::Result;
use limelight::{attribute, Buffer, BufferUsageHint, DrawMode, Program, Renderer, Uniform};
use limelight_yew::{LimelightComponent, LimelightController, ShouldRequestAnimationFrame};

#[attribute]
struct VertexDescription {
    position: [f32; 2],
}

impl VertexDescription {
    pub fn new(x: f32, y: f32) -> Self {
        VertexDescription { position: [x, y] }
    }
}

struct Animation {
    program: Program<VertexDescription, ()>,
    buffer: Buffer<VertexDescription>,
    uniform: Uniform<[f32; 3]>,
}

impl Default for Animation {
    fn default() -> Self {
        let buffer = Buffer::new(vec![], BufferUsageHint::DynamicDraw);
        let uniform = Uniform::new([0., 0., 0.]);

        let program = Program::new(
            include_str!("../shaders/shader.vert"),
            include_str!("../shaders/shader.frag"),
            DrawMode::Triangles,
        )
        .with_uniform("u_color", uniform.clone());

        Animation {
            buffer,
            program,
            uniform,
        }
    }
}

impl LimelightController for Animation {
    fn draw(&mut self, renderer: &mut Renderer, time: f64) -> Result<ShouldRequestAnimationFrame> {
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

        renderer.render(&mut self.program, &self.buffer)?;

        Ok(true)
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<LimelightComponent<Animation>>();
}
