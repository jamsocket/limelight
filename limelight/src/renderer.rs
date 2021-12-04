use crate::{
    program::ProgramLike,
    shadow_gpu::{GpuState, ShadowGpu},
    vertex_attribute::VertexAttribute,
};
use anyhow::{anyhow, Result};
use web_sys::WebGl2RenderingContext;

pub struct Renderer {
    gpu: ShadowGpu,
}

impl Renderer {
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        let gpu = ShadowGpu::new(gl);
        Renderer { gpu }
    }

    pub fn render<T: VertexAttribute>(
        &mut self,
        program: &mut impl ProgramLike<T>,
        //buffer: &impl BufferLike<T>,
    ) -> Result<()> {
        // let state: GpuState = GpuState {
        //     program: Some(program.get_program(&mut self.gpu)?),
        //     buffer: Some(buffer.get_vao(&mut self.gpu)?),
        //     uniforms: todo!(),
        // };

        Ok(())
    }
}
