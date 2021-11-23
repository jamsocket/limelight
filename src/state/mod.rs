use crate::{buffer::AttributeBuffer, program::Program, vertex_attribute::VertexAttribute};

use self::blending::BlendFunc;

pub mod blending;
pub mod enable;

pub struct Stage<T: VertexAttribute> {
    pub blend_func: Option<BlendFunc>,
    pub program: Program<T>,
    pub buffer: AttributeBuffer<T>,
}
