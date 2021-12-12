use limelight::attribute;
use crate::color::Color;

#[attribute]
pub struct Circle {
    center: [f32; 2],
    radius: f32,
    color: Color,
}
