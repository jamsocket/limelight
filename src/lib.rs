pub mod buffer;
pub mod draw_modes;
pub mod gpu_init;
pub mod layer;
pub mod plan;
pub mod program;
pub mod renderer;
pub mod state;
pub mod types;
pub mod uniform;
pub mod vertex_attribute;

pub use bytemuck;
pub use derive_vertex_attribute::vertex_attribute;

#[allow(unused)]
macro_rules! console_log {
    ($($x: expr), +) => (
        web_sys::console::log_1(&wasm_bindgen::JsValue::from(
            format!($($x),+)));
    )
}

#[allow(unused)]
pub(crate) use console_log;
