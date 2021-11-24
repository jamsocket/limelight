use crate::vertex_attribute::{VertexAttribute, VertexAttributeBinding};
use anyhow::{anyhow, Result};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlVertexArrayObject};

enum ProgramInner {
    Uncompiled {
        frag_shader_src: String,
        vert_shader_src: String,
    },
    Compiled {
        program: WebGlProgram,
        vao: WebGlVertexArrayObject,
    },
}

pub struct Program<T: VertexAttribute> {
    inner: RefCell<ProgramInner>,
    _ph: PhantomData<T>,
}

pub trait BindableProgram {
    fn bind(&self, gl: &WebGl2RenderingContext) -> Result<()>;
}

impl<T: VertexAttribute> BindableProgram for Program<T> {
    fn bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        let inner = self.inner.borrow_mut();
        match &*inner {
            ProgramInner::Compiled { program, vao, .. } => {
                gl.use_program(Some(&program));
                gl.bind_vertex_array(Some(&vao));
            }
            ProgramInner::Uncompiled {
                vert_shader_src,
                frag_shader_src,
            } => {
                let frag_shader = compile_shader(gl, ShaderType::FragmentShader, &frag_shader_src)?;
                let vertex_shader = compile_shader(gl, ShaderType::VertexShader, &vert_shader_src)?;

                let program = gl
                    .create_program()
                    .ok_or_else(|| anyhow!("Error creating program."))?;

                gl.attach_shader(&program, &frag_shader);
                gl.attach_shader(&program, &vertex_shader);

                gl.link_program(&program);

                if !gl.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS) {
                    if let Some(info) = gl.get_program_info_log(&program) {
                        return Err(anyhow!("Encountered error linking program: {}", info));
                    } else {
                        return Err(anyhow!("Encountered unknown error linking program."));
                    }
                }

                let vao = gl
                    .create_vertex_array()
                    .ok_or_else(|| anyhow!("Couldn't create vertex array."))?;

                gl.bind_vertex_array(Some(&vao));
                bind_vertex_attributes(&T::describe(), &gl, &program)?;

                drop(inner);
                self.inner.replace(ProgramInner::Compiled {
                    program,
                    vao,
                });
            }
        }

        Ok(())
    }
}

impl<T: VertexAttribute> Program<T> {
    pub fn new(frag_shader: &str, vert_shader: &str) -> Rc<Self> {
        Rc::new(Program {
            inner: RefCell::new(ProgramInner::Uncompiled {
                frag_shader_src: frag_shader.to_string(),
                vert_shader_src: vert_shader.to_string(),
            }),
            _ph: PhantomData::default(),
        })
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

pub fn bind_vertex_attributes(
    bindings: &[VertexAttributeBinding],
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
) -> Result<()> {
    let mut offset: i32 = 0;
    let stride = bindings.iter().map(|d| d.kind.byte_size()).sum();

    for binding in bindings {
        let location = gl.get_attrib_location(program, &binding.variable_name);
        if location == -1 {
            return Err(anyhow!(
                "Expected the program to have a variable called {}, but one was not found.",
                binding.variable_name
            ));
        }

        gl.vertex_attrib_pointer_with_i32(
            location as _,
            binding.kind.size(),
            binding.kind.data_type() as _,
            false,
            stride,
            offset,
        );
        gl.enable_vertex_attrib_array(location as _);

        offset += binding.kind.byte_size();
    }

    Ok(())
}
