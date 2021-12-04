use crate::types::{DataType, SizedDataType};

#[derive(Debug, PartialEq)]
pub struct VertexAttributeBinding {
    pub variable_name: String,
    pub kind: SizedDataType,
}

impl VertexAttributeBinding {
    pub fn new(name: &str, data_type: DataType, size: i32) -> Self {
        VertexAttributeBinding {
            variable_name: name.to_string(),
            kind: SizedDataType::new(data_type, size),
        }
    }
}

pub trait VertexAttribute: bytemuck::Pod + bytemuck::Zeroable {
    fn describe() -> Vec<VertexAttributeBinding>;
}

impl VertexAttribute for () {
    fn describe() -> Vec<VertexAttributeBinding> {
        Vec::new()
    }
}