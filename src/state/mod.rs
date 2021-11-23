use web_sys::WebGlProgram;
use self::blending::BlendFunc;

pub mod blending;
pub mod enable;

#[derive(Default)]
pub struct WebGLState {
    pub blend_func: Option<BlendFunc>,
    pub program: Option<WebGlProgram>,
}
