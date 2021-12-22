use crate::{color::Color, common::{LinePosition, identity_line}};
use anyhow::Result;
use limelight::{
    attribute,
    renderer::Drawable,
    state::{
        blending::{BlendFunction, BlendingFactorDest, BlendingFactorSrc},
        StateDescriptor,
    },
    Buffer, BufferUsageHint, DrawMode, Program, Uniform,
};

#[attribute]
pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub width: f32,
    pub color: Color,
}

pub struct LineLayer {
    lines: Buffer<Line>,
    positions: Buffer<LinePosition>,
    program: Program<LinePosition, Line>,
    transform: Uniform<[[f32; 4]; 4]>,
}

impl Default for LineLayer {
    fn default() -> Self {
        LineLayer::new()
    }
}

impl LineLayer {
    pub fn new() -> Self {
        Self::new_transform(Uniform::identity())
    }

    pub fn new_transform(transform: Uniform<[[f32; 4]; 4]>) -> Self {
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
            positions: Buffer::new(identity_line(), BufferUsageHint::StaticDraw),
            program,
            transform,
        }
    }

    pub fn transform(&self) -> Uniform<[[f32; 4]; 4]> {
        self.transform.clone()
    }

    pub fn buffer(&self) -> Buffer<Line> {
        self.lines.clone()
    }
}

impl Drawable for LineLayer {
    fn draw(&mut self, renderer: &mut limelight::Renderer) -> Result<()> {
        renderer.render_instanced(&mut self.program, &self.positions, &self.lines)?;

        Ok(())
    }
}
