use std::collections::HashMap;

use crate::{
    buffer::BufferLike,
    program::ProgramLike,
    shadow_gpu::{GpuState, ShadowGpu},
    vertex_attribute::VertexAttribute,
};
use anyhow::Result;
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
        buffer: &impl BufferLike<T>,
    ) -> Result<()> {
        let bound_program = program.get_program(&self.gpu)?;

        let mut uniforms = HashMap::new();
        for (uniform_handle, uniform) in &bound_program.uniforms {
            uniforms.insert(uniform_handle.clone(), uniform.get_value());
        }

        let state: GpuState = GpuState {
            program: Some(bound_program.handle()),
            buffer: buffer.get_buffer(),
            uniforms,
        };

        self.gpu
            .draw_arrays(&state, program.draw_mode(), 0, buffer.len() as _)?;
        Ok(())
    }
}
