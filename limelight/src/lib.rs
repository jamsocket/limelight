#![doc = include_str!("../README.md")]

pub mod buffer;
pub mod draw_modes;
pub mod program;
pub mod renderer;
pub mod shadow_gpu;
pub mod state;
pub mod types;
pub mod uniform;
pub mod attribute;

pub use bytemuck;
pub use limelight_derive::{attribute, Attribute};

pub use buffer::{Buffer, DummyBuffer};
pub use shadow_gpu::BufferUsageHint;
pub use program::Program;
pub use renderer::Renderer;
pub use draw_modes::DrawMode;
pub use uniform::Uniform;
pub use attribute::{Attribute, AttributeBinding};

#[allow(unused)]
macro_rules! console_log {
    ($($x: expr), +) => (
        web_sys::console::log_1(&wasm_bindgen::JsValue::from(
            format!($($x),+)));
    )
}

#[allow(unused)]
pub(crate) use console_log;
