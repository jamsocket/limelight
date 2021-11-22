use web_sys::{WebGlBuffer, WebGlProgram, WebGlTexture};

use self::blending::BlendFunc;

pub mod blending;
pub mod enable;

#[derive(Default)]
pub struct WebGLState {
    pub blend_func: Option<BlendFunc>,
    pub program: Option<WebGlProgram>,

    // Buffer mount points.
    pub array_buffer: Option<WebGlBuffer>,
    pub index_buffer: Option<WebGlBuffer>,

    // Texture mount points.
    pub texture_2d: Option<WebGlTexture>,
}
