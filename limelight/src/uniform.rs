use crate::shadow_gpu::{UniformValue, UniformValueType};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct Uniform<T: UniformValueType> {
    value: Rc<RefCell<T>>,
}

impl<T: UniformValueType> Uniform<T> {
    pub fn new(value: T) -> Uniform<T> {
        Uniform {
            value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn set_value(&self, value: T) {
        *self.value.borrow_mut() = value
    }
}

pub trait GenericUniform {
    fn get_value(&self) -> UniformValue;
}

impl<T: UniformValueType> GenericUniform for Uniform<T> {
    fn get_value(&self) -> UniformValue {
        UniformValueType::into_uniform_value(&*self.value.borrow())
    }
}
