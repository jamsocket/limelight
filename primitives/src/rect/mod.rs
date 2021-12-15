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

use crate::color::Color;

#[attribute]
pub struct Rect {
    pub upper_left: [f32; 2],
    pub lower_right: [f32; 2],
    pub color: Color,
}

pub struct RectLayer {
    rects: Buffer<Rect>,
    program: Program<(), Rect>,
}

impl RectLayer {
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
        .with_uniform("u_transform", transform);

        RectLayer {
            rects: Buffer::new_empty(BufferUsageHint::DynamicDraw),
            program,
        }
    }

    pub fn buffer(&self) -> Buffer<Rect> {
        self.rects.clone()
    }
}

impl Drawable for RectLayer {
    fn draw(&mut self, renderer: &mut limelight::Renderer) -> Result<()> {
        renderer.render_instanced(&mut self.program, &DummyBuffer::new(4), &self.rects)?;

        Ok(())
    }
}
