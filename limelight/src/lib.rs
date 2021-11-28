#![doc = include_str!("../README.md")]

pub mod buffer;
pub mod draw_modes;
pub mod gpu_init;
pub mod program;
pub mod renderer;
pub mod state;
pub mod types;
pub mod uniform;
pub mod vertex_attribute;

pub use bytemuck;
pub use limelight_derive::{vertex_attribute, VertexAttribute};

pub use buffer::attribute_buffer::AttributeBuffer;
pub use buffer::dummy::DummyBuffer;
pub use buffer::types::BufferUsageHint;
pub use draw_modes::DrawMode;
pub use program::{GlProgram, Program};
pub use renderer::Renderer;
pub use uniform::{Uniform, UniformHandle};
pub use vertex_attribute::{VertexAttribute, VertexAttributeBinding};

#[allow(unused)]
macro_rules! console_log {
    ($($x: expr), +) => (
        web_sys::console::log_1(&wasm_bindgen::JsValue::from(
            format!($($x),+)));
    )
}

#[allow(unused)]
pub(crate) use console_log;
