use crate::color::Color;
use anyhow::Result;
use limelight::{
    attribute,
    renderer::Drawable,
    state::{
        blending::{BlendFunction, BlendingFactorDest, BlendingFactorSrc},
        StateDescriptor,
    },
    Buffer, BufferUsageHint, DrawMode, DummyBuffer, Program, Uniform,
};

#[attribute]
pub struct Circle {
    pub position: [f32; 2],
    pub radius: f32,
    pub color: Color,
}

pub struct CircleLayer {
    circles: Buffer<Circle>,
    program: Program<(), Circle>,
}

impl CircleLayer {
    pub fn new(transform: Uniform<[[f32; 4]; 4]>) -> Self {
        let program = Program::new(
            include_str!("shader.vert"),
            include_str!("shader.frag"),
            DrawMode::TriangleStrip,
        )
        .with_state(StateDescriptor {
            blend_func: Some(BlendFunction {
                source_factor: BlendingFactorSrc::One,
                dst_factor: BlendingFactorDest::OneMinusSrcAlpha,
                ..Default::default()
            }),
            ..Default::default()
        })
        .with_uniform("u_transform", transform);

        CircleLayer {
            circles: Buffer::new_empty(BufferUsageHint::DynamicDraw),
            program,
        }
    }

    pub fn buffer(&self) -> Buffer<Circle> {
        self.circles.clone()
    }
}

impl Drawable for CircleLayer {
    fn draw(&mut self, renderer: &mut limelight::Renderer) -> Result<()> {
        renderer.render_instanced(&mut self.program, &DummyBuffer::new(4), &self.circles)?;

        Ok(())
    }
}
