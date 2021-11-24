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
    console_log,
};
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;

pub struct Stage {
    #[allow(unused)]
    state: State,
    draw_mode: DrawMode,

    program: Box<dyn BindableProgram>,
    buffer: Box<dyn BindableBuffer>,
}

impl Stage {
    pub fn new<T: VertexAttribute>(
        program: Rc<Program<T>>,
        buffer: Rc<AttributeBuffer<T>>,
        state: State,
        draw_mode: DrawMode,
    ) -> Self {
        
        Stage {
            state,
            draw_mode,
            buffer: buffer.boxed_clone(),
            program: program.boxed_clone(),
        }
    }
}

#[derive(Debug)]
pub enum RenderStep {
    Enable(EnableCap),
    Disable(EnableCap),
    SetBlendEquation(BlendEquation),
    SetBlendFactor(BlendingFactorSrc, BlendingFactorDest),
    SetProgram(Box<dyn BindableProgram>),
    SetBuffer(Box<dyn BindableBuffer>),
    DrawArrays(Box<dyn BindableBuffer>, DrawMode),
}

impl RenderStep {
    pub fn apply(&self, gl: &WebGl2RenderingContext) {
        //console_log!("Executing stage: {:?}", self);

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
            //console_log!("Running task {:?}", task);
            task.apply(gl);

            let error = gl.get_error();
            if error != WebGl2RenderingContext::NO_ERROR {
                panic!("WebGL Error: {:?}", error);
            }
        }
    }

    pub fn new(stages: Vec<Stage>) -> Self {
        let mut plan: Vec<RenderStep> = Vec::new();

        for stage in stages {
            plan.push(RenderStep::SetBuffer(stage.buffer.boxed_clone()));
            plan.push(RenderStep::SetProgram(stage.program));
            plan.push(RenderStep::DrawArrays(stage.buffer, stage.draw_mode));
        }

        Renderer { plan }
    }
}
