#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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

#[derive(Default)]
pub struct BlendFunc {
    pub source_factor: BlendingFactorSrc,
    pub dst_factor: BlendingFactorDest,
    pub equation: BlendEquation,
}
