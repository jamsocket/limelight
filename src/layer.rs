use web_sys::WebGlProgram;

use crate::vertex_attribute::VertexAttribute;

trait VecLayer {
    type AttributeData: VertexAttribute;
    // type UniformData

    fn program() -> WebGlProgram;
}
