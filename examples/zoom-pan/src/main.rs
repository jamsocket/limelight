use anyhow::Result;
use limelight::{attribute, Buffer, BufferUsageHint, DrawMode, Program, Renderer};
use limelight_transform::TransformUniform;
use limelight_yew::{LimelightComponent, LimelightController};

struct ZoomPan {
    program: Program<VertexDescription, ()>,
    buffer: Buffer<VertexDescription>,
    transform: TransformUniform,
}

impl Default for ZoomPan {
    fn default() -> Self {
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
            include_str!("../shaders/shader.vert"),
            include_str!("../shaders/shader.frag"),
            DrawMode::Triangles,
        )
        .with_uniform("u_transform", transform.uniform());

        ZoomPan {
            buffer,
            program,
            transform,
        }
    }
}

impl LimelightController for ZoomPan {
    fn draw(
        &mut self,
        renderer: &mut Renderer,
        _ts: f64,
    ) -> Result<limelight_yew::ShouldRequestAnimationFrame> {
        renderer.render(&mut self.program, &self.buffer)?;
        Ok(false)
    }

    fn handle_drag(&mut self, x: f32, y: f32) -> limelight_yew::ShouldRequestAnimationFrame {
        self.transform.pan((x, y));
        true
    }

    fn handle_scroll(
        &mut self,
        x_amount: f32,
        y_amount: f32,
        _x_position: f32,
        _y_position: f32,
    ) -> limelight_yew::ShouldRequestAnimationFrame {
        self.transform.pan((x_amount, y_amount));
        true
    }

    fn handle_zoom(
        &mut self,
        amount: f32,
        x: f32,
        y: f32,
    ) -> limelight_yew::ShouldRequestAnimationFrame {
        let scale_factor = 1. + amount / 100.;
        self.transform.scale(scale_factor, (x, y));
        true
    }
}

#[attribute]
struct VertexDescription {
    position: [f32; 2],
}

impl VertexDescription {
    pub fn new(x: f32, y: f32) -> Self {
        VertexDescription { position: [x, y] }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<LimelightComponent<ZoomPan>>();
}
