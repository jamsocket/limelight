/// Enum of WebGL Bind Points.
///
/// Each bind point is a global bind point in WebGL that can have an
/// array bound to it.
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum BufferBindPoint {
    ArrayBuffer = 0x8892,
    ElementArrayBuffer = 0x8893,
}

/// Usage hint to tell WebGL how a buffer will be used.
///
/// These hints are non-binding; you can read/write from a
/// buffer as much as you like regardless of the hint. However,
/// a driver may use the hint to optimize memory layout.
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum BufferUsageHint {
    /// Hint that a buffer is written to once and read once.
    StreamDraw = 0x88E0,

    /// Hint that a buffer is written to once and ready many times.
    StaticDraw = 0x88E4,

    /// Hint that a buffer is written and read many times.
    DynamicDraw = 0x88E8,
}
