pub mod buffer;
pub mod plan;
pub mod state;
pub mod types;
pub mod vertex_attribute;
pub mod layer;
pub mod program;
pub mod draw_modes;
pub use bytemuck;

pub use derive_vertex_attribute::vertex_attribute;

#[allow(unused)]
macro_rules! console_log {
    ($($x: expr), +) => (
        web_sys::console::log_1(&wasm_bindgen::JsValue::from(
            format!($($x),+)));
    )
}

pub(crate) use console_log;
