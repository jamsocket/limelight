use std::{rc::Rc};
use anyhow::Result;
use web_sys::{WebGlVertexArrayObject, WebGl2RenderingContext};
use super::{GpuBind, BufferHandle};

#[derive(Clone)]
pub struct VaoHandle {
    vao: Rc<WebGlVertexArrayObject>,
    buffers: Vec<BufferHandle>,
}

impl VaoHandle {
    pub fn soft_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        // VAO is already bound, but we check on whether the buffers are fresh.

        for buffer in self.buffers {
            // pass
        }

        Ok(())
    }
}

impl PartialEq for VaoHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.vao, &other.vao)
    }
}

impl GpuBind for Option<VaoHandle> {
    fn gpu_bind(&self, gl: &web_sys::WebGl2RenderingContext) -> anyhow::Result<()> {
        if let Some(vh) = &*self {
            gl.bind_vertex_array(Some(&*vh.vao));
        } else {
            gl.bind_vertex_array(None);
        }        

        Ok(())
    }
}