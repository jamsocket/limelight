#[derive(Copy, Clone)]
#[repr(u32)]
pub enum CullingMode {
    Front = 0x0404,
    Back = 0x0405,
    FrontAndBack = 0x0408,
}

