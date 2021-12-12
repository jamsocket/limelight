use limelight::{
    attribute,
    webgl::types::{DataType, SizedDataType},
    AsSizedDataType,
};

use crate::color::Color;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal = 0x0,
    Vertical = 0x1,
}

unsafe impl bytemuck::Pod for Orientation {}
unsafe impl bytemuck::Zeroable for Orientation {}

impl AsSizedDataType for Orientation {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType::new(DataType::UnsignedInt, 1)
    }
}

#[attribute]
pub struct Hairline {
    location: f32,
    color: Color,
    axis: Orientation,
}
