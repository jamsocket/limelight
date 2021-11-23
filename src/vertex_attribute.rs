use crate::types::{DataType, SizedDataType};
pub use derive_vertex_attribute::VertexAttribute;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

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

pub fn bind_vertex_attributes(
    bindings: &[VertexAttributeBinding],
    gl: &WebGl2RenderingContext,
    program: &WebGlProgram,
) {
    let mut offset: i32 = 0;
    let stride = bindings.iter().map(|d| d.kind.byte_size()).sum();

    for binding in bindings {
        let location = gl.get_attrib_location(program, &binding.variable_name);
        if location == -1 {
            panic!(
                "Expected the program to have a variable called {}, but one was not found.",
                binding.variable_name
            );
        }

        gl.vertex_attrib_pointer_with_i32(
            location as _,
            binding.kind.size(),
            binding.kind.data_type() as _,
            false,
            stride,
            offset,
        );
        gl.enable_vertex_attrib_array(location as _);

        offset += binding.kind.byte_size();
    }
}
