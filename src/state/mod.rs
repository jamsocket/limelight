use crate::{buffer::AttributeBuffer, program::Program, vertex_attribute::VertexAttribute};

use self::blending::BlendFunc;

pub mod blending;
pub mod enable;

#[derive(Default)]
pub struct WebGLState<T: VertexAttribute> {
    pub blend_func: Option<BlendFunc>,
    pub program: Option<Program<T>>,

    pub buffer: Option<AttributeBuffer<T>>,
}
