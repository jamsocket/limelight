use crate::{color::Color, common::{RelativePosition, identity_quad}};
use anyhow::Result;
use limelight::{
    attribute,
    renderer::Drawable,
    state::{
        blending::{BlendFunction, BlendingFactorDest, BlendingFactorSrc},
        StateDescriptor,
    },
    webgl::types::{DataType, SizedDataType},
    AsSizedDataType, Buffer, BufferUsageHint, DrawMode, Program, Uniform,
};

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal = 0x0,
    Vertical = 0x1,
}

unsafe impl bytemuck::Pod for Orientation {}
unsafe impl bytemuck::Zeroable for Orientation {}

impl AsSizedDataType for Orientation {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType::new(DataType::UnsignedInt, 1)
    }
}

#[attribute]
pub struct Hairline {
    pub location: f32,
    pub color: Color,
    pub orientation: Orientation,
}

impl Default for HairlineLayer {
    fn default() -> Self {
        HairlineLayer::new()
    }
}

pub struct HairlineLayer {
    lines: Buffer<Hairline>,
    positions: Buffer<RelativePosition>,
    program: Program<RelativePosition, Hairline>,
    transform: Uniform<[[f32; 4]; 4]>,
}

impl HairlineLayer {
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

        HairlineLayer {
            lines: Buffer::new_empty(BufferUsageHint::DynamicDraw),
            positions: Buffer::new(identity_quad(), BufferUsageHint::StaticDraw),
            program,
            transform,
        }
    }

    pub fn buffer(&self) -> Buffer<Hairline> {
        self.lines.clone()
    }

    pub fn transform(&self) -> Uniform<[[f32; 4]; 4]> {
        self.transform.clone()
    }
}

impl Drawable for HairlineLayer {
    fn draw(&mut self, renderer: &mut limelight::Renderer) -> Result<()> {
        renderer.render_instanced(&mut self.program, &self.positions, &self.lines)?;

        Ok(())
    }
}
