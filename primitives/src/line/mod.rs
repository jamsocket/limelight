use anyhow::Result;
use limelight::{
    attribute, renderer::Drawable, Buffer, BufferUsageHint, DrawMode, DummyBuffer, Program, Uniform, state::{StateDescriptor, blending::{BlendFunction, BlendingFactorSrc, BlendingFactorDest}},
};
use crate::color::Color;

#[attribute]
pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub width: f32,
    pub color: Color,
}

pub struct LineLayer {
    lines: Buffer<Line>,
    program: Program<(), Line>,
}

impl LineLayer {
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
        .with_uniform("u_transform", transform.clone());

        LineLayer {
            lines: Buffer::new_empty(BufferUsageHint::DynamicDraw),
            program,
        }
    }

    pub fn buffer(&self) -> Buffer<Line> {
        self.lines.clone()
    }
}

impl Drawable for LineLayer {
    fn draw(&mut self, renderer: &mut limelight::Renderer) -> Result<()> {
        renderer.render_instanced(&mut self.program, &DummyBuffer::new(4), &self.lines)?;

        Ok(())
    }
}
