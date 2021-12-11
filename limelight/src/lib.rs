#![doc = include_str!("../README.md")]

mod attribute;
pub mod buffer;
pub mod draw_modes;
pub mod program;
pub mod renderer;
pub mod shadow_gpu;
pub mod state;
pub mod uniform;
pub mod webgl;

pub use bytemuck;
pub use limelight_derive::{attribute, Attribute};

pub use attribute::{Attribute, AttributeBinding};
pub use buffer::{Buffer, DummyBuffer};
pub use draw_modes::DrawMode;
pub use program::Program;
pub use renderer::Renderer;
pub use uniform::Uniform;
pub use webgl::types::AsSizedDataType;
pub use webgl::buffer::{BufferBindPoint, BufferUsageHint};

#[allow(unused)]
macro_rules! console_log {
    ($($x: expr), +) => (
        web_sys::console::log_1(&wasm_bindgen::JsValue::from(
            format!($($x),+)));
    )
}

#[allow(unused)]
pub(crate) use console_log;
