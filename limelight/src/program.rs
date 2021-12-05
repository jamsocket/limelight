use anyhow::Result;
use std::{
    collections::{hash_map::Entry, HashMap},
    marker::PhantomData,
};

use crate::{
    shadow_gpu::{ProgramHandle, ShadowGpu, UniformHandle, UniformValueType},
    uniform::GenericUniform,
    DrawMode, Uniform, Attribute,
};

pub trait ProgramLike<T: Attribute, I: Attribute> {
    fn get_program(&mut self, gpu: &ShadowGpu) -> Result<&BoundProgram<T>>;

    fn draw_mode(&self) -> DrawMode;
}

pub struct BoundProgram<T: Attribute> {
    handle: ProgramHandle,
    pub uniforms: Vec<(UniformHandle, Box<dyn GenericUniform>)>,
    draw_mode: DrawMode,
    _ph: PhantomData<T>,
}

impl<T: Attribute> BoundProgram<T> {
    pub fn handle(&self) -> ProgramHandle {
        self.handle.clone()
    }
}

pub struct UnboundProgram<T: Attribute> {
    fragment_shader_source: String,
    vertex_shader_source: String,
    uniforms: HashMap<String, Box<dyn GenericUniform>>,
    draw_mode: DrawMode,
    _ph: PhantomData<T>,
}

impl<T: Attribute> UnboundProgram<T> {
    pub fn with_uniform<U: UniformValueType>(
        &mut self,
        name: &str,
        uniform: Uniform<U>,
    ) -> &mut Self {
        match self.uniforms.entry(name.to_string()) {
            Entry::Occupied(_) => panic!("Tried to set uniform {} more than once.", name),
            Entry::Vacant(e) => e.insert(Box::new(uniform.clone())),
        };

        self
    }

    fn new_dummy() -> Self {
        UnboundProgram {
            _ph: PhantomData::default(),
            fragment_shader_source: "".to_string(),
            vertex_shader_source: "".to_string(),
            uniforms: HashMap::new(),
            draw_mode: DrawMode::Triangles,
        }
    }

    pub fn bind(self, gpu: &ShadowGpu) -> Result<BoundProgram<T>> {
        let vertex_shader = gpu.compile_vertex_shader(&self.vertex_shader_source)?;
        let fragment_shader = gpu.compile_fragment_shader(&self.fragment_shader_source)?;

        let attribute_locations: Vec<String> = T::describe()
            .iter()
            .map(|d| d.variable_name.to_string())
            .collect();

        let program = gpu.link_program(&fragment_shader, &vertex_shader, &attribute_locations)?;

        let mut bound_uniforms = Vec::with_capacity(self.uniforms.len());

        for (name, uniform) in self.uniforms {
            let loc = gpu.get_uniform_handle(&program, &name)?;
            bound_uniforms.push((loc, uniform));
        }

        Ok(BoundProgram {
            handle: program.clone(),
            uniforms: bound_uniforms,
            draw_mode: self.draw_mode,
            _ph: PhantomData::default(),
        })
    }
}

pub enum Program<T: Attribute> {
    Unbound(UnboundProgram<T>),
    Bound(BoundProgram<T>),
}

impl<T: Attribute> Program<T> {
    pub fn new(
        fragment_shader_source: &str,
        vertex_shader_source: &str,
        draw_mode: DrawMode,
    ) -> Self {
        Program::Unbound(UnboundProgram {
            fragment_shader_source: fragment_shader_source.to_string(),
            vertex_shader_source: vertex_shader_source.to_string(),
            uniforms: HashMap::new(),
            draw_mode,
            _ph: PhantomData::default(),
        })
    }
}

impl<T: Attribute, I: Attribute> ProgramLike<T, I> for BoundProgram<T> {
    fn get_program(&mut self, _gpu: &ShadowGpu) -> Result<&BoundProgram<T>> {
        Ok(self)
    }

    fn draw_mode(&self) -> DrawMode {
        self.draw_mode
    }
}

impl<T: Attribute> Program<T> {
    pub fn with_uniform<U: UniformValueType>(mut self, name: &str, uniform: Uniform<U>) -> Self {
        match &mut self {
            Program::Bound(_) => {
                panic!("Tried calling with_uniform on a program that is already bound.")
            }
            Program::Unbound(p) => {
                p.with_uniform(name, uniform);
            }
        }

        self
    }
}

impl<T: Attribute, I: Attribute> ProgramLike<T, I> for Program<T> {
    fn get_program(&mut self, gpu: &ShadowGpu) -> Result<&BoundProgram<T>> {
        match self {
            Program::Bound(p) => Ok(p),
            Program::Unbound(p) => {
                let mut dummy_program = UnboundProgram::new_dummy();
                std::mem::swap(&mut dummy_program, p);

                let result = dummy_program.bind(gpu)?;
                *self = Program::Bound(result);
                
                match self {
                    Program::Bound(result) => Ok(result),
                    _ => panic!()
                }
            }
        }
    }

    fn draw_mode(&self) -> DrawMode {
        match self {
            Program::Bound(p) => p.draw_mode,
            Program::Unbound(p) => p.draw_mode,
        }
    }
}
