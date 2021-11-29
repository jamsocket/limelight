use slice_of_array::SliceFlatExt;
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

#[derive(Clone, Copy, PartialEq)]
pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat2([[f32; 2]; 2]),
    Mat3([[f32; 3]; 3]),
    Mat4([[f32; 4]; 4]),
    Int(i32),
    IntVec2([i32; 2]),
    IntVec3([i32; 3]),
    IntVec4([i32; 4]),
    UnsignedInt(u32),
    UnsignedIntVec2([u32; 2]),
    UnsignedIntVec3([u32; 3]),
    UnsignedIntVec4([u32; 4]),

    // TODO: non-square matrices are supported by WebGL2:
    // https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniformMatrix
}

impl UniformValue {
    pub fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        match self {
            UniformValue::Float(v) => gl.uniform1f(Some(location), *v),
            UniformValue::Vec2(v) => gl.uniform2fv_with_f32_array(Some(location), v),
            UniformValue::Vec3(v) => gl.uniform3fv_with_f32_array(Some(location), v),
            UniformValue::Vec4(v) => gl.uniform4fv_with_f32_array(Some(location), v),

            UniformValue::Int(v) => gl.uniform1i(Some(location), *v),
            UniformValue::IntVec2(v) => gl.uniform2iv_with_i32_array(Some(location), v),
            UniformValue::IntVec3(v) => gl.uniform3iv_with_i32_array(Some(location), v),
            UniformValue::IntVec4(v) => gl.uniform4iv_with_i32_array(Some(location), v),

            UniformValue::UnsignedInt(v) => gl.uniform1ui(Some(location), *v),
            UniformValue::UnsignedIntVec2(v) => gl.uniform2uiv_with_u32_array(Some(location), v),
            UniformValue::UnsignedIntVec3(v) => gl.uniform3uiv_with_u32_array(Some(location), v),
            UniformValue::UnsignedIntVec4(v) => gl.uniform4uiv_with_u32_array(Some(location), v),

            UniformValue::Mat2(v) => gl.uniform_matrix2fv_with_f32_array(Some(location), false, v.flat()),
            UniformValue::Mat3(v) => gl.uniform_matrix3fv_with_f32_array(Some(location), false, v.flat()),
            UniformValue::Mat4(v) => gl.uniform_matrix4fv_with_f32_array(Some(location), false, v.flat()),
        }
    }
}

pub trait UniformValueType: Clone + 'static {
    fn into_uniform_value(v: &Self) -> UniformValue;
}

impl UniformValueType for f32 {
    fn into_uniform_value(v: &f32) -> UniformValue {
        UniformValue::Float(*v)
    }
}
