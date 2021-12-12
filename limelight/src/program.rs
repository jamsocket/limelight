use anyhow::Result;
use std::{
    collections::{hash_map::Entry, HashMap},
    marker::PhantomData,
};

use crate::{
    shadow_gpu::{AttributeInfo, ProgramHandle, ShadowGpu, UniformHandle, UniformValueType},
    uniform::GenericUniform,
    Attribute, DrawMode, Uniform, state::{StateDescriptor},
};

pub trait ProgramLike<T: Attribute, I: Attribute> {
    fn get_program(&mut self, gpu: &ShadowGpu) -> Result<&BoundProgram<T, I>>;

    fn globals(&self) -> StateDescriptor;

    fn draw_mode(&self) -> DrawMode;
}

pub struct BoundProgram<T: Attribute, I: Attribute> {
    handle: ProgramHandle,
    pub uniforms: Vec<(UniformHandle, Box<dyn GenericUniform>)>,
    draw_mode: DrawMode,
    state: StateDescriptor,
    _ph: PhantomData<T>,
    _phi: PhantomData<I>,
}

impl<T: Attribute, I: Attribute> BoundProgram<T, I> {
    pub fn handle(&self) -> ProgramHandle {
        self.handle.clone()
    }

    pub fn attributes(&self) -> &HashMap<String, AttributeInfo> {
        &self.handle.attributes
    }
}

pub struct UnboundProgram<T: Attribute, I: Attribute> {
    fragment_shader_source: String,
    vertex_shader_source: String,
    uniforms: HashMap<String, Box<dyn GenericUniform>>,
    draw_mode: DrawMode,
    state: StateDescriptor,
    _ph: PhantomData<T>,
    _phi: PhantomData<I>,
}

impl<T: Attribute, I: Attribute> UnboundProgram<T, I> {
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
            _phi: PhantomData::default(),
            fragment_shader_source: "".to_string(),
            vertex_shader_source: "".to_string(),
            uniforms: HashMap::new(),
            state: StateDescriptor::default(),
            draw_mode: DrawMode::Triangles,
        }
    }

    pub fn bind(self, gpu: &ShadowGpu) -> Result<BoundProgram<T, I>> {
        let vertex_shader = gpu.compile_vertex_shader(&self.vertex_shader_source)?;
        let fragment_shader = gpu.compile_fragment_shader(&self.fragment_shader_source)?;
        let program = gpu.link_program(&fragment_shader, &vertex_shader)?;

        let mut bound_uniforms = Vec::with_capacity(self.uniforms.len());

        for (name, uniform) in self.uniforms {
            let loc = gpu.get_uniform_handle(&program, &name)?;
            bound_uniforms.push((loc, uniform));
        }

        Ok(BoundProgram {
            handle: program.clone(),
            uniforms: bound_uniforms,
            draw_mode: self.draw_mode,
            state: self.state,
            _ph: PhantomData::default(),
            _phi: PhantomData::default(),
        })
    }
}

pub enum Program<T: Attribute, I: Attribute> {
    Unbound(UnboundProgram<T, I>),
    Bound(BoundProgram<T, I>),
}

impl<T: Attribute, I: Attribute> Program<T, I> {
    pub fn new(
        vertex_shader_source: &str,
        fragment_shader_source: &str,
        draw_mode: DrawMode,
    ) -> Self {
        Program::Unbound(UnboundProgram {
            fragment_shader_source: fragment_shader_source.to_string(),
            vertex_shader_source: vertex_shader_source.to_string(),
            uniforms: HashMap::new(),
            draw_mode,
            state: StateDescriptor::default(),
            _ph: PhantomData::default(),
            _phi: PhantomData::default(),
        })
    }
}

impl<T: Attribute, I: Attribute> ProgramLike<T, I> for BoundProgram<T, I> {
    fn get_program(&mut self, _gpu: &ShadowGpu) -> Result<&BoundProgram<T, I>> {
        Ok(self)
    }

    fn globals(&self) -> StateDescriptor {
        self.state.clone()
    }

    fn draw_mode(&self) -> DrawMode {
        self.draw_mode
    }
}

impl<T: Attribute, I: Attribute> Program<T, I> {
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

    pub fn with_state(mut self, state: StateDescriptor) -> Self {
        match &mut self {
            Program::Bound(_) => {
                panic!("Tried calling with_uniform on a program that is already bound.")
            }
            Program::Unbound(p) => {
                p.state = state;
            }
        }

        self
    }
}

impl<T: Attribute, I: Attribute> ProgramLike<T, I> for Program<T, I> {
    fn get_program(&mut self, gpu: &ShadowGpu) -> Result<&BoundProgram<T, I>> {
        match self {
            Program::Bound(p) => Ok(p),
            Program::Unbound(p) => {
                let mut dummy_program = UnboundProgram::new_dummy();
                std::mem::swap(&mut dummy_program, p);

                let result = dummy_program.bind(gpu)?;
                *self = Program::Bound(result);

                match self {
                    Program::Bound(result) => Ok(result),
                    _ => panic!(),
                }
            }
        }
    }

    fn globals(&self) -> StateDescriptor {
        match self {
            Program::Bound(b) => b.state.clone(),
            Program::Unbound(b) => b.state.clone(),
        }
    }

    fn draw_mode(&self) -> DrawMode {
        match self {
            Program::Bound(p) => p.draw_mode,
            Program::Unbound(p) => p.draw_mode,
        }
    }
}
