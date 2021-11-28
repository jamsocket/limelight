use self::attribute_buffer::BufferHandle;
use crate::{GpuBind, VertexAttribute};
use anyhow::Result;
use web_sys::WebGl2RenderingContext;

pub mod attribute_buffer;
pub mod dummy;
pub mod types;

pub trait BufferLike<T: VertexAttribute>: GpuBind {
    fn len(&self) -> usize;
}

impl<T: VertexAttribute> GpuBind for BufferHandle<T> {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        self.as_ref().gpu_bind(gl)
    }
}

impl<T: VertexAttribute> BufferLike<T> for BufferHandle<T> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}
