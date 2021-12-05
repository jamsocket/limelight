use crate::types::{DataType, SizedDataType};

#[derive(Debug, PartialEq)]
pub struct AttributeBinding {
    pub variable_name: String,
    pub kind: SizedDataType,
}

impl AttributeBinding {
    pub fn new(name: &str, data_type: DataType, size: i32) -> Self {
        AttributeBinding {
            variable_name: name.to_string(),
            kind: SizedDataType::new(data_type, size),
        }
    }
}

pub trait Attribute: bytemuck::Pod + bytemuck::Zeroable {
    fn describe() -> Vec<AttributeBinding>;
}

impl Attribute for () {
    fn describe() -> Vec<AttributeBinding> {
        Vec::new()
    }
}