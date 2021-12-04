use self::{buffer::BufferHandle};
use crate::{types::SizedDataType, DrawMode};
use anyhow::{anyhow, Result};
use std::{borrow::Borrow, collections::HashMap, rc::Rc};
pub use uniforms::{UniformValue, UniformValueType, UniformHandle};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

mod buffer;
#[allow(unused)]
mod types;
mod uniforms;

trait GpuBind {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()>;
}

pub struct FragmentShader(WebGlShader);
pub struct VertexShader(WebGlShader);

#[derive(Clone)]
pub struct ProgramHandle(Rc<WebGlProgram>);

impl GpuBind for Option<ProgramHandle> {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        if let Some(ProgramHandle(program)) = &self {
            gl.use_program(Some(program.borrow()));
        } else {
            gl.use_program(None);
        }

        Ok(())
    }
}

impl PartialEq for ProgramHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[derive(Default)]
pub struct GpuState {
    pub program: Option<ProgramHandle>,
    pub buffer: Option<BufferHandle>,
    pub uniforms: HashMap<UniformHandle, UniformValue>,
}

pub struct ShadowGpu {
    gl: WebGl2RenderingContext,
    state: GpuState,
}

impl ShadowGpu {
    pub fn new(gl: WebGl2RenderingContext) -> Self {
        ShadowGpu {
            gl,
            state: GpuState::default(),
        }
    }

    pub fn draw_arrays(
        &mut self,
        state: &mut GpuState,
        mode: DrawMode,
        first: i32,
        count: i32,
    ) -> Result<()> {
        self.set_state(state)?;
        self.gl.draw_arrays(mode as _, first, count);
        Ok(())
    }

    pub fn get_uniform_handle(&self, program: &ProgramHandle, name: &str) -> Result<UniformHandle> {
        let location = self
            .gl
            .get_uniform_location(&program.0, name)
            .ok_or_else(|| anyhow!("Uniform {} not found.", name))?;

        Ok(UniformHandle::new(location))
    }

    fn set_state(&mut self, new_state: &mut GpuState) -> Result<()> {
        // Program
        if self.state.program != new_state.program {
            new_state.program.gpu_bind(&self.gl)?;
            self.state.program = new_state.program.clone();
        }

        if self.state.buffer != new_state.buffer {
            new_state.buffer.gpu_bind(&self.gl)?;
            self.state.buffer = new_state.buffer.clone();
        } else if let Some(buffer) = &new_state.buffer {
            buffer.sync_data(&self.gl)?;
        }

        // Uniforms
        for (location, value) in &new_state.uniforms {
            if let Some(v) = self.state.uniforms.get(location) {
                if v == value {
                    continue;
                }
            }

            self.state.uniforms.insert(location.clone(), value.clone());
            value.bind(&self.gl, location);
        }

        Ok(())
    }

    pub fn create_buffer(&mut self, attributes: &[SizedDataType]) -> BufferHandle {
        BufferHandle::new(attributes)
    }

    pub fn link_program(
        &mut self,
        frag_shader: &FragmentShader,
        vertex_shader: &VertexShader,
        attribute_locations: &[String],
    ) -> Result<ProgramHandle> {
        let gl_program = self
            .gl
            .create_program()
            .ok_or_else(|| anyhow!("Error creating program."))?;

        self.gl.attach_shader(&gl_program, &frag_shader.0);
        self.gl.attach_shader(&gl_program, &vertex_shader.0);

        for (location, attr) in attribute_locations.iter().enumerate() {
            self.gl
                .bind_attrib_location(&gl_program, location as _, attr);
        }

        self.gl.link_program(&gl_program);

        if !self
            .gl
            .get_program_parameter(&gl_program, WebGl2RenderingContext::LINK_STATUS)
        {
            if let Some(info) = self.gl.get_program_info_log(&gl_program) {
                return Err(anyhow!("Encountered error linking program: {}", info));
            } else {
                return Err(anyhow!("Encountered unknown error linking program."));
            }
        }

        Ok(ProgramHandle(Rc::new(gl_program)))
    }

    fn compile_shader(&self, shader_type: ShaderType, source: &str) -> Result<WebGlShader> {
        let shader = self
            .gl
            .create_shader(shader_type as _)
            .ok_or_else(|| anyhow!("Could not create shader."))?;
        self.gl.shader_source(&shader, source);
        self.gl.compile_shader(&shader);

        if self
            .gl
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(self
                .gl
                .get_shader_info_log(&shader)
                .map(|d| anyhow!("Error compiling shader {}", d))
                .unwrap_or_else(|| anyhow!("Unknown error compiling shader.")))
        }
    }

    pub fn compile_fragment_shader(&self, source: &str) -> Result<FragmentShader> {
        Ok(FragmentShader(
            self.compile_shader(ShaderType::FragmentShader, source)?,
        ))
    }

    pub fn compile_vertex_shader(&self, source: &str) -> Result<VertexShader> {
        Ok(VertexShader(
            self.compile_shader(ShaderType::VertexShader, source)?,
        ))
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

#[derive(Copy, Clone)]
#[repr(u32)]
enum ShaderType {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
}
