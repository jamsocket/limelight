use crate::{
    draw_modes::DrawMode,
    gpu_init::{GpuBind, GpuInit},
    uniform::{BindableUniform, Uniform, UniformValue},
    vertex_attribute::VertexAttribute,
};
use anyhow::{anyhow, Result};
use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
    marker::PhantomData,
    rc::Rc,
};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

pub struct Program<T: VertexAttribute> {
    frag_shader_src: String,
    vert_shader_src: String,
    uniforms: HashMap<String, Box<dyn BindableUniform>>,
    draw_mode: DrawMode,
    _ph: PhantomData<T>,
}

impl<T: VertexAttribute> GpuInit for Program<T> {
    type Output = GlProgram<T>;

    fn gpu_init(self, gl: &WebGl2RenderingContext) -> Result<GlProgram<T>> {
        let frag_shader = compile_shader(gl, ShaderType::FragmentShader, &self.frag_shader_src)?;
        let vertex_shader = compile_shader(gl, ShaderType::VertexShader, &self.vert_shader_src)?;

        let program = gl
            .create_program()
            .ok_or_else(|| anyhow!("Error creating program."))?;

        gl.attach_shader(&program, &frag_shader);
        gl.attach_shader(&program, &vertex_shader);

        for (location, attr) in T::describe().iter().enumerate() {
            gl.bind_attrib_location(&program, location as _, &attr.variable_name);
        }

        gl.link_program(&program);
        gl.use_program(Some(&program));

        if !gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS) {
            if let Some(info) = gl.get_program_info_log(&program) {
                return Err(anyhow!("Encountered error linking program: {}", info));
            } else {
                return Err(anyhow!("Encountered unknown error linking program."));
            }
        }

        let mut uniforms = Vec::new();
        for (name, value) in self.uniforms {
            let location = gl
                .get_uniform_location(&program, &name)
                .ok_or_else(|| anyhow!("Could not find uniform with name {}", name))?;

            uniforms.push((location, value));
        }

        Ok(GlProgram {
            program,
            uniforms,
            draw_mode: self.draw_mode,
            _ph: PhantomData::default(),
        })
    }
}

impl<T: VertexAttribute> Program<T> {
    pub fn new(frag_shader_src: &str, vert_shader_src: &str, draw_mode: DrawMode) -> Self {
        Program {
            frag_shader_src: frag_shader_src.to_string(),
            vert_shader_src: vert_shader_src.to_string(),
            uniforms: HashMap::new(),
            draw_mode,
            _ph: PhantomData::default(),
        }
    }

    pub fn with_uniform<U: UniformValue>(
        mut self,
        name: &str,
        uniform: Rc<Uniform<U>>,
    ) -> Self {
        match self.uniforms.entry(name.to_string()) {
            Entry::Occupied(_) => panic!("Tried to set uniform {} more than once.", name),
            Entry::Vacant(e) => e.insert(Box::new(uniform)),
        };
        self
    }
}

#[derive(Debug)]
pub struct GlProgram<T: VertexAttribute> {
    program: WebGlProgram,
    uniforms: Vec<(WebGlUniformLocation, Box<dyn BindableUniform>)>,
    draw_mode: DrawMode,
    _ph: PhantomData<T>,
}

impl<T: VertexAttribute> GpuBind for GlProgram<T> {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        gl.use_program(Some(&self.program));

        // Bind uniforms.
        for (location, uniform) in &self.uniforms {
            uniform.bind(gl, location);
        }

        Ok(())
    }
}

impl<T: VertexAttribute> GlProgram<T> {
    pub fn draw_mode(&self) -> DrawMode {
        self.draw_mode
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
enum ShaderType {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: ShaderType,
    source: &str,
) -> Result<WebGlShader> {
    let shader = gl
        .create_shader(shader_type as _)
        .ok_or_else(|| anyhow!("Could not create shader."))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .map(|d| anyhow!("Error compiling shader {}", d))
            .unwrap_or_else(|| anyhow!("Unknown error compiling shader.")))
    }
}
