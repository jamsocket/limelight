use anyhow::Result;
use web_sys::WebGl2RenderingContext;

pub trait GpuInit {
    type Output: GpuBind;

    fn gpu_init(self, gl: &WebGl2RenderingContext) -> Result<Self::Output>;
}

pub trait GpuBind {
    fn gpu_bind(&mut self, gl: &WebGl2RenderingContext) -> Result<()>;
}
