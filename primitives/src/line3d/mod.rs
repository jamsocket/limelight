use crate::color::Color;
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
pub struct Line3D {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub width: f32,
    pub color: Color,
}

#[attribute]
struct Index {
    index: u32,
}

pub struct Line3DLayer {
    lines: Buffer<Line3D>,
    indices: Buffer<Index>,
    program: Program<Index, Line3D>,
    transform: Uniform<[[f32; 4]; 4]>,
}

impl Default for Line3DLayer {
    fn default() -> Self {
        Line3DLayer::new()
    }
}

impl Line3DLayer {
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

        let indices = Buffer::new(vec![
            Index {
                index: 0
            },
            Index {
                index: 1
            },
            Index {
                index: 2
            },
            Index {
                index: 3
            },
        ], BufferUsageHint::StaticDraw);

        Line3DLayer {
            lines: Buffer::new_empty(BufferUsageHint::DynamicDraw),
            program,
            transform,
            indices,
        }
    }

    pub fn transform(&self) -> Uniform<[[f32; 4]; 4]> {
        self.transform.clone()
    }

    pub fn buffer(&self) -> Buffer<Line3D> {
        self.lines.clone()
    }
}

impl Drawable for Line3DLayer {
    fn draw(&mut self, renderer: &mut limelight::Renderer) -> Result<()> {
        renderer.render_instanced(&mut self.program, &self.indices, &self.lines)?;

        Ok(())
    }
}
