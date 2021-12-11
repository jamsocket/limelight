use limelight::types::{DataType, SizedDataType};
use limelight::{attribute, Attribute, AttributeBinding};

#[attribute]
struct SimpleStruct {
    data: i32,
}

#[test]
fn test_describe_simple_derive() {
    assert_eq!(
        vec![AttributeBinding {
            variable_name: "data".to_string(),
            kind: SizedDataType::new(DataType::Int, 1)
        }],
        SimpleStruct::describe()
    );
}

#[attribute]
struct TwoFieldStruct {
    data1: f32,
    data2: u16,
    data3: i16,
}

#[test]
fn test_describe_two_field_derive() {
    assert_eq!(
        vec![
            AttributeBinding::new("data1", DataType::Float, 1),
            AttributeBinding::new("data2", DataType::UnsignedShort, 1),
            AttributeBinding::new("data3", DataType::Short, 1),
        ],
        TwoFieldStruct::describe()
    );
}

#[attribute]
struct ArrayStruct {
    array: [f32; 4],
}

#[test]
fn test_describe_array_field_derive() {
    assert_eq!(
        vec![AttributeBinding::new("array", DataType::Float, 4)],
        ArrayStruct::describe()
    );
}

#[attribute]
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
            AttributeBinding::new("a1", DataType::UnsignedShort, 4),
            AttributeBinding::new("b1", DataType::Short, 4),
            AttributeBinding::new("c1", DataType::UnsignedByte, 1),
            AttributeBinding::new("d1", DataType::Byte, 3),
            AttributeBinding::new("e1", DataType::UnsignedShort, 2),
        ],
        MultipleArrayStruct::describe()
    );
}
