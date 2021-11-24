use crate::{
    buffer::{AttributeBuffer, BindableBuffer},
    draw_modes::DrawMode,
    program::{BindableProgram, Program},
    state::{
        blending::{BlendEquation, BlendingFactorDest, BlendingFactorSrc},
        enable::EnableCap,
        State,
    },
    vertex_attribute::VertexAttribute,
};
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

pub struct Stage {
    state: State,
    draw_mode: DrawMode,

    program: Rc<Box<dyn BindableProgram>>,
    buffer: Rc<Box<dyn BindableBuffer>>,
}

impl Stage {
    pub fn new<T: VertexAttribute>(
        program: Rc<Program<T>>,
        buffer: Rc<AttributeBuffer<T>>,
        state: State,
        draw_mode: DrawMode,
    ) -> Self {
        todo!()
    }
}

pub enum RenderStep {
    Enable(EnableCap),
    Disable(EnableCap),
    SetBlendEquation(BlendEquation),
    SetBlendFactor(BlendingFactorSrc, BlendingFactorDest),
    SetProgram(Rc<Box<dyn BindableProgram>>),
    SetBuffer(Rc<Box<dyn BindableBuffer>>),
    DrawArrays(Rc<Box<dyn BindableBuffer>>, DrawMode),
}

impl RenderStep {
    pub fn apply(&self, gl: &WebGl2RenderingContext) {
        match self {
            Self::Enable(cap) => gl.enable(*cap as _),
            Self::Disable(cap) => gl.disable(*cap as _),
            Self::SetBlendEquation(eq) => gl.blend_equation(*eq as _),
            Self::SetBlendFactor(sfactor, dfactor) => gl.blend_func(*sfactor as _, *dfactor as _),
            Self::SetProgram(program) => program.bind(gl).unwrap(),
            Self::SetBuffer(buffer) => buffer.bind(gl),
            Self::DrawArrays(buffer, draw_mode) => {
                gl.draw_arrays(*draw_mode as _, 0, buffer.len() as _)
            }
        }
    }
}

pub struct Renderer {
    plan: Vec<RenderStep>,
}

impl Renderer {
    pub fn render(&self, gl: &WebGl2RenderingContext) {
        for task in &self.plan {
            task.apply(gl);
        }
    }

    pub fn new(stages: Vec<Stage>) -> Self {
        let mut plan: Vec<RenderStep> = Vec::new();

        for stage in stages {
            plan.push(RenderStep::SetProgram(stage.program));
            plan.push(RenderStep::SetBuffer(stage.buffer.clone()));
            plan.push(RenderStep::DrawArrays(stage.buffer, stage.draw_mode));
        }

        Renderer { plan }
    }
}
