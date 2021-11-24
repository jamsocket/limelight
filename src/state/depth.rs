#[derive(Copy, Clone)]
#[repr(u32)]
pub enum DepthFunction {
    Never = 0x0200,
    Less = 0x0201,
    Equal = 0x0202,
    LessOrEqual = 0x0203,
    Greater = 0x0204,
    NotEqual = 0x0205,
    GreaterOrEqual = 0x0206,
    Always = 0x0207,
}
