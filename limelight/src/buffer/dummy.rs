use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::{GpuBind, VertexAttribute, VertexAttributeBinding};

use super::BufferLike;

pub struct DummyBuffer {
    size: usize,
}

impl DummyBuffer {
    pub fn new(size: usize) -> Self {
        DummyBuffer { size }
    }
}

impl BufferLike<()> for DummyBuffer {
    fn len(&self) -> usize {
        self.size
    }
}

impl GpuBind for DummyBuffer {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        gl.bind_vertex_array(None);
        Ok(())
    }
}

impl VertexAttribute for () {
    fn describe() -> Vec<VertexAttributeBinding> {
        vec![]
    }
}
