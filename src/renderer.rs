use crate::{
    buffer::AttributeBuffer, gpu_init::GpuBind, program::GlProgram,
    vertex_attribute::VertexAttribute,
};
use anyhow::{anyhow, Result};
use web_sys::WebGl2RenderingContext;

pub struct Renderer {
    gl: WebGl2RenderingContext,
}

impl Renderer {
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        Renderer { gl }
    }

    pub fn render<T: VertexAttribute>(
        &self,
        program: &mut GlProgram<T>,
        buffer: &mut AttributeBuffer<T>,
    ) -> Result<()> {
        buffer.gpu_bind(&self.gl)?;
        program.gpu_bind(&self.gl)?;
        crate::console_log!("Pre draw.");
        self.gl
            .draw_arrays(program.draw_mode() as _, 0, buffer.len() as _);
        crate::console_log!("Post draw.");

        Ok(())
    }

    pub fn get_error(&self) -> Result<()> {
        let error = self.gl.get_error();
        if error != WebGl2RenderingContext::NO_ERROR {
            Err(anyhow!("WebGL Error: {:?}", error))
        } else {
            Ok(())
        }
    }
}
