use limelight::{vertex_attribute, Program};

// Color information can be:
// - Global color (from uniform)
// - Per-element color ([u8; 4])
// - Per-element value, colorized in shader (f32)
// For now, support per-element color only.

pub type Color = [u8; 4];

#[vertex_attribute]
pub struct Line {
    start: [f32; 2],
    end: [f32; 2],
    thickness: f32,
    color: Color,
}

#[vertex_attribute]
pub struct Rect {
    upper_left: [f32; 2],
    lower_right: [f32; 2],
    color: Color,
}

#[vertex_attribute]
pub struct Circle {
    center: [f32; 2],
    radius: f32,
    color: Color,
}

pub const AXIS_HORIZONTAL: u32 = 0x0;
pub const AXIS_VERTICAL: u32 = 0x1;

#[vertex_attribute]
pub struct Hairline {
    location: f32,
    color: Color,
    axis: u32,
}

pub fn line_program() -> Program<Line> {
    todo!()
}
