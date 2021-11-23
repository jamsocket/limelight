use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::state::{
    blending::{BlendEquation, BlendingFactorDest, BlendingFactorSrc},
    enable::EnableCap,
};

pub enum WebGlStateChange {
    Enable(EnableCap),
    Disable(EnableCap),
    SetBlendEquation(BlendEquation),
    SetBlendFactor(BlendingFactorSrc, BlendingFactorDest),
    SetProgram(Option<WebGlProgram>),
}

impl WebGlStateChange {
    pub fn apply(&self, gl: &WebGl2RenderingContext) {
        match self {
            Self::Enable(cap) => gl.enable(*cap as _),
            Self::Disable(cap) => gl.disable(*cap as _),
            Self::SetBlendEquation(eq) => gl.blend_equation(*eq as _),
            Self::SetBlendFactor(sfactor, dfactor) => gl.blend_func(*sfactor as _, *dfactor as _),
            Self::SetProgram(program) => gl.use_program(program.as_ref()),
        }
    }
}
