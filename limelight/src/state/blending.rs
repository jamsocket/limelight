use web_sys::WebGl2RenderingContext;

use crate::shadow_gpu::GpuBind;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum BlendingFactorDest {
    Zero = 0,
    One = 1,
    SrcColor = 0x0300,
    OneMinusSrcColor = 0x0301,
    SrcAlpha = 0x0302,
    OneMinusSrcAlpha = 0x0303,
    DstAlpha = 0x0304,
    OneMinusDstAlpha = 0x0305,
}

impl Default for BlendingFactorDest {
    fn default() -> Self {
        BlendingFactorDest::Zero
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum BlendingFactorSrc {
    Zero = 0,
    One = 1,
    DstColor = 0x0306,
    OneMinusDstColor = 0x0307,
    SrcAlphaSaturate = 0x0308,
    SrcAlpha = 0x0302,
    OneMinusSrcAlpha = 0x0303,
    DstAlpha = 0x0304,
    OneMinusDstAlpha = 0x0305,
}

impl Default for BlendingFactorSrc {
    fn default() -> Self {
        BlendingFactorSrc::One
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum BlendEquation {
    Add = 0x8006,
    BlendEquation = 0x8009,
    BlendEquationAlpha = 0x883d,
    Subtract = 0x800a,
    ReverseSubtract = 0x800b,
}

impl Default for BlendEquation {
    fn default() -> Self {
        BlendEquation::Add
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct BlendFunction {
    pub source_factor: BlendingFactorSrc,
    pub dst_factor: BlendingFactorDest,
    pub equation: BlendEquation,
}

impl GpuBind for Option<BlendFunction> {
    fn gpu_bind(&self, gl: &web_sys::WebGl2RenderingContext) -> anyhow::Result<()> {
        match self {
            Some(blend) => {
                gl.blend_func(blend.source_factor as _, blend.dst_factor as _);
                gl.blend_equation(blend.equation as _);
                gl.enable(WebGl2RenderingContext::BLEND);        
            },
            None => gl.disable(WebGl2RenderingContext::BLEND),
        }
        
        Ok(())
    }
}