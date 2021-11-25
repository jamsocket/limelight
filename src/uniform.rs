use std::{cell::RefCell, fmt::Debug, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlUniformLocation};

#[derive(Debug)]
pub struct Uniform<T: UniformValue> {
    value: RefCell<T>,
}

impl<T: UniformValue> Uniform<T> {
    pub fn new(value: T) -> Rc<Self> {
        Rc::new(Self {
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

impl UniformValue for f32 {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform1f(Some(location), *self);
    }
}

impl UniformValue for [f32; 2] {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform2f(Some(location), self[0], self[1]);
    }
}

// TODO: implement more uniform types.
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL2RenderingContext/uniform
// https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform

impl UniformValue for i32 {
    fn bind(&self, gl: &WebGl2RenderingContext, location: &WebGlUniformLocation) {
        gl.uniform1i(Some(location), *self);
    }
}
