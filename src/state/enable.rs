#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum EnableCap {
    CullFace = 0x0B44,
    Blend = 0x0BE2,
    Dither = 0x0BD0,
    StencilTest = 0x0B90,
    DepthTest = 0x0B71,
    ScissorTest = 0x0C11,
    PolygonOffsetFill = 0x8037,
    SampleAlphaToCoverage = 0x809E,
    SampleCoverage = 0x80A0,
}
