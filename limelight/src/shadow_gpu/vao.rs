use crate::webgl::types::DataType;
use std::collections::BTreeMap;

use super::{buffer::BindResult, state::BufferBinding, BufferHandle};
use anyhow::anyhow;
use web_sys::WebGlVertexArrayObject;

pub struct VaoHandle {
    pub vao: Option<WebGlVertexArrayObject>,
    pub buffers: BTreeMap<BufferHandle, Vec<BufferBinding>>,
}

impl VaoHandle {
    pub fn gpu_bind(&mut self, gl: &web_sys::WebGl2RenderingContext) -> anyhow::Result<()> {
        let create = if let Some(vao) = &self.vao {
            gl.bind_vertex_array(Some(vao));
            false
        } else {
            log::info!("Creating Vertex Array.");
            let vao = gl
                .create_vertex_array()
                .ok_or_else(|| anyhow!("Couldn't create vertex array."))?;
            gl.bind_vertex_array(Some(&vao));
            self.vao = Some(vao);
            true
        };

        for (buffer, bindings) in &self.buffers {
            let upsized_buffer = match buffer.bind(gl)? {
                BindResult::BoundExisting => false,
                BindResult::BoundNew => true,
            };

            if !create && !upsized_buffer {
                // If this is not a new VAO, and the buffer already existed,
                // we don't need to update the bindings.
                continue;
            }
            log::info!("Updating or creating initial bindings: {:?}", bindings);

            for binding in bindings {
                match binding.kind.data_type() {
                    DataType::Float => gl.vertex_attrib_pointer_with_i32(
                        binding.location,
                        binding.kind.size(),
                        binding.kind.data_type() as _,
                        binding.normalized,
                        binding.stride,
                        binding.offset,
                    ),
                    _ => gl.vertex_attrib_i_pointer_with_i32(
                        binding.location,
                        binding.kind.size(),
                        binding.kind.data_type() as _,
                        binding.stride,
                        binding.offset,
                    ),
                }

                if binding.divisor != 0 {
                    gl.vertex_attrib_divisor(binding.location, binding.divisor);
                }

                gl.enable_vertex_attrib_array(binding.location);
            }
        }

        Ok(())
    }
}
