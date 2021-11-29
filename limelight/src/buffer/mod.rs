use std::rc::Rc;

use crate::{VertexAttribute, shadow_gpu::{ShadowGpu}};
use anyhow::Result;

pub mod attribute_buffer;
pub mod types;

pub trait BufferLike<T: VertexAttribute> {
    fn len(&self) -> usize;

    fn get_vao(&self, gl: &ShadowGpu) -> Result<BufferHandle>;
}
