use limelight::types::{DataType, SizedDataType};
use limelight::{vertex_attribute, VertexAttribute, VertexAttributeBinding};

#[vertex_attribute]
struct SimpleStruct {
    data: i32,
}

#[test]
fn test_describe_simple_derive() {
    assert_eq!(
        vec![VertexAttributeBinding {
            variable_name: "data".to_string(),
            kind: SizedDataType::new(DataType::Int, 1)
        }],
        SimpleStruct::describe()
    );
}

#[vertex_attribute]
struct TwoFieldStruct {
    data1: f32,
    data2: u16,
    data3: i16,
}

#[test]
fn test_describe_two_field_derive() {
    assert_eq!(
        vec![
            VertexAttributeBinding::new("data1", DataType::Float, 1),
            VertexAttributeBinding::new("data2", DataType::UnsignedShort, 1),
            VertexAttributeBinding::new("data3", DataType::Short, 1),
        ],
        TwoFieldStruct::describe()
    );
}

#[vertex_attribute]
struct ArrayStruct {
    array: [f32; 4],
}

#[test]
fn test_describe_array_field_derive() {
    assert_eq!(
        vec![VertexAttributeBinding::new("array", DataType::Float, 4)],
        ArrayStruct::describe()
    );
}

#[vertex_attribute]
struct MultipleArrayStruct {
    a1: [u16; 4],
    b1: [i16; 4],
    c1: u8,
    d1: [i8; 3],
    e1: [u16; 2],
}

#[test]
fn test_describe_multiple_array_field_derive() {
    assert_eq!(
        vec![
            VertexAttributeBinding::new("a1", DataType::UnsignedShort, 4),
            VertexAttributeBinding::new("b1", DataType::Short, 4),
            VertexAttributeBinding::new("c1", DataType::UnsignedByte, 1),
            VertexAttributeBinding::new("d1", DataType::Byte, 3),
            VertexAttributeBinding::new("e1", DataType::UnsignedShort, 2),
        ],
        MultipleArrayStruct::describe()
    );
}
