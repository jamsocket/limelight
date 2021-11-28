use slice_of_array::SliceFlatExt;
use std::{cell::RefCell, fmt::Debug, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

#[derive(Debug)]
pub struct Uniform<T: UniformValue> {
    value: RefCell<T>,
}

pub type UniformHandle<T> = Rc<Uniform<T>>;

impl<T: UniformValue> Uniform<T> {
    pub fn new(value: T) -> UniformHandle<T> {
        Rc::new(Uniform {
            value: RefCell::new(value),
        })
    }

    pub fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        self.value.borrow().bind(gl, location);
    }

    pub fn set_value(&self, value: T) {
        *self.value.borrow_mut() = value
    }
}

pub trait BindableUniform: core::fmt::Debug {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation);

    fn boxed_clone(&self) -> Box<dyn BindableUniform>;
}

impl<T: UniformValue> BindableUniform for Rc<Uniform<T>> {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        self.value.borrow().bind(gl, location)
    }

    fn boxed_clone(&self) -> Box<dyn BindableUniform> {
        Box::new(self.clone())
    }
}

pub trait UniformValue: bytemuck::Pod + bytemuck::Zeroable + Debug {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation);
}

/// UniformValue for f32 -> float
impl UniformValue for f32 {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform1f(Some(location), *self);
    }
}

/// UniformValue for [f32; 2] -> vec2
impl UniformValue for [f32; 2] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform2f(Some(location), self[0], self[1]);
    }
}

/// UniformValue for [f32; 3] -> vec3
impl UniformValue for [f32; 3] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform3f(Some(location), self[0], self[1], self[2]);
    }
}

/// UniformValue for [f32; 4] -> vec4
impl UniformValue for [f32; 4] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform4f(Some(location), self[0], self[1], self[2], self[3]);
    }
}

/// UniformValue for [[f32; 2]; 2] -> mat2
impl UniformValue for [[f32; 2]; 2] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform_matrix2fv_with_f32_array(Some(location), false, self.flat());
    }
}

/// UniformValue for [[f32; 3]; 3] -> mat3
impl UniformValue for [[f32; 3]; 3] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform_matrix3fv_with_f32_array(Some(location), false, self.flat());
    }
}

/// UniformValue for [[f32; 4]; 4] -> mat4
impl UniformValue for [[f32; 4]; 4] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform_matrix4fv_with_f32_array(Some(location), false, self.flat());
    }
}

/// UniformValue for i32 -> int
impl UniformValue for i32 {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform1i(Some(location), *self);
    }
}

/// UniformValue for [i32; 2] -> ivec2
impl UniformValue for [i32; 2] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform2i(Some(location), self[0], self[1]);
    }
}

/// UniformValue for [i32; 3] -> ivec3
impl UniformValue for [i32; 3] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform3i(Some(location), self[0], self[1], self[2]);
    }
}

/// UniformValue for [i32; 4] -> ivec4
impl UniformValue for [i32; 4] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform4i(Some(location), self[0], self[1], self[2], self[3]);
    }
}

/// UniformValue for u32 -> uint
impl UniformValue for u32 {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform1ui(Some(location), *self);
    }
}

/// UniformValue for [u32; 2] -> uvec2
impl UniformValue for [u32; 2] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform2ui(Some(location), self[0], self[1]);
    }
}

/// UniformValue for [u32; 3] -> uvec3
impl UniformValue for [u32; 3] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform3ui(Some(location), self[0], self[1], self[2]);
    }
}

/// UniformValue for [u32; 4] -> uvec4
impl UniformValue for [u32; 4] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform4ui(Some(location), self[0], self[1], self[2], self[3]);
    }
}
