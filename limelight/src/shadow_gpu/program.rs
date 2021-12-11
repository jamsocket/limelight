use super::{AttributeInfo, GpuBind};
use anyhow::Result;
use std::borrow::Borrow;
use std::{collections::HashMap, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlProgram};

#[derive(Clone)]
pub struct ProgramHandle {
    pub program: Rc<WebGlProgram>,

    /// A map from attribute name to attribute location in the program.
    pub attributes: HashMap<String, AttributeInfo>,
}

impl GpuBind for Option<ProgramHandle> {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        if let Some(ProgramHandle { program, .. }) = &self {
            gl.use_program(Some(program.borrow()));
        } else {
            gl.use_program(None);
        }

        Ok(())
    }
}

impl PartialEq for ProgramHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.program, &other.program)
    }
}
