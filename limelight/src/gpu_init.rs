use anyhow::Result;
use web_sys::WebGl2RenderingContext;

pub trait GpuBind {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()>;
}
