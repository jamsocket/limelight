use std::collections::{BTreeMap, HashMap};

use crate::{
    attribute::Attribute,
    buffer::BufferLike,
    program::ProgramLike,
    shadow_gpu::{AttributeInfo, BufferBinding, BufferHandle, GpuState, ShadowGpu},
};
use anyhow::Result;
use web_sys::WebGl2RenderingContext;

pub struct Renderer {
    gpu: ShadowGpu,
}

enum DrawCall {
    DrawArrays {
        first: usize,
        count: usize,
    },
    DrawArraysInstanced {
        first: usize,
        count: usize,
        instances: usize,
    },
}

struct BufferBindingGroup {
    bindings: BTreeMap<BufferHandle, Vec<BufferBinding>>,
    attributes: HashMap<String, AttributeInfo>,
}

impl BufferBindingGroup {
    fn new(attributes: HashMap<String, AttributeInfo>) -> Self {
        Self {
            attributes,
            bindings: BTreeMap::new(),
        }
    }

    fn add_buffer<T: Attribute>(&mut self, buffer: &impl BufferLike<T>, divisor: u32) {
        if let Some(buffer) = buffer.get_buffer() {
            let stride = T::describe().into_iter().map(|d| d.kind.byte_size()).sum();
            let mut offset = 0;
            let mut bindings = Vec::new();

            for attribute in T::describe() {
                if let Some(program_binding) = self.attributes.get(&attribute.variable_name) {
                    if attribute.kind != program_binding.kind.as_sized_type() {
                        // panic!("The variable {} has type {:?} as an attribute, but {:?} in the program definition.", attribute.variable_name, attribute.kind, program_binding.kind);
                        crate::console_log!("The variable {} has type {:?} as an attribute, but {:?} in the program definition.", attribute.variable_name, attribute.kind, program_binding.kind);
                    }

                    bindings.push(BufferBinding {
                        kind: attribute.kind,
                        divisor,
                        location: program_binding.location as _,
                        normalized: false,
                        offset,
                        stride,
                    });

                    offset += attribute.kind.byte_size();
                } else {
                    crate::console_log!(
                        "Attribute has variable {}, which isn't used in the program.",
                        attribute.variable_name
                    );
                }
            }

            if bindings.is_empty() {
                crate::console_log!("No attributes in the buffer overlapped with the program.");
            } else {
                self.bindings.insert(buffer, bindings);
            }
        }
    }
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

        buffers: BTreeMap<BufferHandle, Vec<BufferBinding>>,
    ) -> Result<()> {
        let bound_program = program.get_program(&self.gpu)?;

        let mut uniforms = HashMap::new();
        for (uniform_handle, uniform) in &bound_program.uniforms {
            uniforms.insert(uniform_handle.clone(), uniform.get_value());
        }

        let state: GpuState = GpuState {
            program: Some(bound_program.handle()),
            buffers,
            uniforms,
            globals: program.globals(),
        };

        match draw_call {
            DrawCall::DrawArrays { count, first } => {
                self.gpu
                    .draw_arrays(&state, program.draw_mode(), first as _, count as _)?
            }
            DrawCall::DrawArraysInstanced {
                first,
                count,
                instances,
            } => self.gpu.draw_arrays_instanced(
                &state,
                program.draw_mode(),
                first as _,
                count as _,
                instances as _,
            )?,
        }

        Ok(())
    }

    pub fn render<T: Attribute>(
        &mut self,
        program: &mut impl ProgramLike<T, ()>,
        vertex_buffer: &impl BufferLike<T>,
    ) -> Result<()> {
        let bound_program = program.get_program(&self.gpu)?;
        let program_attributes = bound_program.attributes();

        let mut bg = BufferBindingGroup::new(program_attributes.clone());
        bg.add_buffer(vertex_buffer, 0);

        self.render_impl(
            DrawCall::DrawArrays {
                first: 0,
                count: vertex_buffer.len(),
            },
            program,
            bg.bindings,
        )
    }

    pub fn render_instanced<T: Attribute, I: Attribute>(
        &mut self,
        program: &mut impl ProgramLike<T, I>,
        vertex_buffer: &impl BufferLike<T>,
        instance_buffer: &impl BufferLike<I>,
    ) -> Result<()> {
        let bound_program = program.get_program(&self.gpu)?;
        let program_attributes = bound_program.attributes();

        let mut bg = BufferBindingGroup::new(program_attributes.clone());
        bg.add_buffer(vertex_buffer, 0);
        bg.add_buffer(instance_buffer, 1);

        self.render_impl(
            DrawCall::DrawArraysInstanced {
                first: 0,
                count: vertex_buffer.len(),
                instances: instance_buffer.len(),
            },
            program,
            bg.bindings,
        )
    }
}

pub trait Drawable {
    fn draw(&mut self, renderer: &mut Renderer) -> Result<()>;
}
