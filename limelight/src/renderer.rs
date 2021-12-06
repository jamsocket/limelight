use std::{collections::HashMap};

use crate::{
    buffer::{BufferLike, VertexAttribute, InstanceAttribute},
    program::ProgramLike,
    shadow_gpu::{GpuState, ShadowGpu, VaoHandle},
    attribute::Attribute,
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

trait VaoLike<T: Attribute, I: Attribute> {
    fn get_vao(&self) -> VaoHandle;
}

impl Renderer {
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        let gpu = ShadowGpu::new(gl);
        Renderer { gpu }
    }

    fn render_impl<T: Attribute, I: Attribute>(
        &mut self,
        draw_call: DrawCall,
        program: &mut impl ProgramLike<T, I>,
        vao: &mut impl VaoLike<T, I>,
    ) -> Result<()> {
        let bound_program = program.get_program(&self.gpu)?;

        let mut uniforms = HashMap::new();
        for (uniform_handle, uniform) in &bound_program.uniforms {
            uniforms.insert(uniform_handle.clone(), uniform.get_value());
        }

        let state: GpuState = GpuState {
            program: Some(bound_program.handle()),
            vao: Some(vao.get_vao()),
            uniforms,
        };

        // match draw_call {
        //     DrawCall::DrawArrays => {
        //         self.gpu
        //             .draw_arrays(&state, program.draw_mode(), 0, buffer.len() as _)?
        //     }
        //     DrawCall::DrawArraysInstanced { instances } => self.gpu.draw_arrays_instanced(
        //         &state,
        //         program.draw_mode(),
        //         0,
        //         buffer.len() as _,
        //         instances as _,
        //     )?,
        // }

        Ok(())
    }

    pub fn render<T: Attribute>(
        &mut self,
        program: &mut impl ProgramLike<T, ()>,
        vao: &impl VaoLike<T, ()>,
    ) -> Result<()> {
        // self.render_impl(DrawCall::DrawArrays, program, buffer)
        Ok(())
    }

    pub fn render_instanced<T: Attribute, I: Attribute>(
        &mut self,
        program: &mut impl ProgramLike<T, I>,
        vertex_buffer: &impl BufferLike<T, VertexAttribute>,
        instance_buffer: &impl BufferLike<I, InstanceAttribute>,
        instances: usize,
    ) -> Result<()> {
        //self.render_impl(DrawCall::DrawArraysInstanced { instances }, program, vertex_buffer)
        Ok(())
    }
}

pub trait Drawable {
    fn draw(renderer: &mut Renderer) -> Result<()>;
}
