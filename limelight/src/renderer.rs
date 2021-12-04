use std::{collections::HashMap};

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

enum DrawCall {
    DrawArrays,
    DrawArraysInstanced { instances: usize },
}

impl Renderer {
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        let gpu = ShadowGpu::new(gl);
        Renderer { gpu }
    }

    fn render_impl<T: VertexAttribute>(
        &mut self,
        draw_call: DrawCall,
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

        match draw_call {
            DrawCall::DrawArrays => {
                self.gpu
                    .draw_arrays(&state, program.draw_mode(), 0, buffer.len() as _)?
            }
            DrawCall::DrawArraysInstanced { instances } => self.gpu.draw_arrays_instanced(
                &state,
                program.draw_mode(),
                0,
                buffer.len() as _,
                instances as _,
            )?,
        }

        Ok(())
    }

    pub fn render<T: VertexAttribute>(
        &mut self,
        program: &mut impl ProgramLike<T>,
        buffer: &impl BufferLike<T>,
    ) -> Result<()> {
        self.render_impl(DrawCall::DrawArrays, program, buffer)
    }

    pub fn render_instanced<T: VertexAttribute>(
        &mut self,
        program: &mut impl ProgramLike<T>,
        buffer: &impl BufferLike<T>,
        instances: usize,
    ) -> Result<()> {
        self.render_impl(DrawCall::DrawArraysInstanced { instances }, program, buffer)
    }
}

pub trait Drawable {
    fn draw(renderer: &mut Renderer) -> Result<()>;
}
