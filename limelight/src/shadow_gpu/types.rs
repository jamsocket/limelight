use crate::types::{DataType, SizedDataType};

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

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum UniformType {
    Byte = 0x1400,
    UnsignedByte = 0x1401,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Int = 0x1404,
    UnsignedInt = 0x1405,
    Float = 0x1406,
    FloatVec2 = 0x8B50,
    FloatVec3 = 0x8B51,
    FloatVec4 = 0x8B52,
    IntVec2 = 0x8B53,
    IntVec3 = 0x8B54,
    IntVec4 = 0x8B55,
    Bool = 0x8B56,
    BoolVec2 = 0x8B57,
    BoolVec3 = 0x8B58,
    BoolVec4 = 0x8B59,
    FloatMat2 = 0x8B5A,
    FloatMat3 = 0x8B5B,
    FloatMat4 = 0x8B5C,
    Sampler2D = 0x8B5E,
    SamplerCube = 0x8B60,
}

impl UniformType {
    pub fn as_sized_type(&self) -> SizedDataType {
        match self {
            UniformType::Float => SizedDataType::new(crate::types::DataType::Float, 1),
            UniformType::FloatVec2 => SizedDataType::new(crate::types::DataType::Float, 2),
            UniformType::FloatVec3 => SizedDataType::new(crate::types::DataType::Float, 3),
            UniformType::FloatVec4 => SizedDataType::new(crate::types::DataType::Float, 4),
            UniformType::IntVec2 => todo!(),
            UniformType::IntVec3 => todo!(),
            UniformType::IntVec4 => todo!(),
            UniformType::Bool => todo!(),
            UniformType::BoolVec2 => todo!(),
            UniformType::BoolVec3 => todo!(),
            UniformType::BoolVec4 => todo!(),
            UniformType::FloatMat2 => todo!(),
            UniformType::FloatMat3 => todo!(),
            UniformType::FloatMat4 => todo!(),
            UniformType::Sampler2D => todo!(),
            UniformType::SamplerCube => todo!(),
            UniformType::Byte => todo!(),
            UniformType::UnsignedByte => todo!(),
            UniformType::Short => todo!(),
            UniformType::UnsignedShort => todo!(),
            UniformType::Int => SizedDataType::new(DataType::Int, 1),
            UniformType::UnsignedInt => SizedDataType::new(DataType::UnsignedInt, 1),
        }
    }
}

impl TryFrom<u32> for UniformType {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1400 => Ok(UniformType::Byte),
            0x1401 => Ok(UniformType::UnsignedByte),
            0x1402 => Ok(UniformType::Short),
            0x1403 => Ok(UniformType::UnsignedShort),
            0x1404 => Ok(UniformType::Int),
            0x1405 => Ok(UniformType::UnsignedInt),
            0x1406 => Ok(UniformType::Float),
            0x8B50 => Ok(UniformType::FloatVec2),
            0x8B51 => Ok(UniformType::FloatVec3),
            0x8B52 => Ok(UniformType::FloatVec4),
            0x8B53 => Ok(UniformType::IntVec2),
            0x8B54 => Ok(UniformType::IntVec3),
            0x8B55 => Ok(UniformType::IntVec4),
            0x8B56 => Ok(UniformType::Bool),
            0x8B57 => Ok(UniformType::BoolVec2),
            0x8B58 => Ok(UniformType::BoolVec3),
            0x8B59 => Ok(UniformType::BoolVec4),
            0x8B5A => Ok(UniformType::FloatMat2),
            0x8B5B => Ok(UniformType::FloatMat3),
            0x8B5C => Ok(UniformType::FloatMat4),
            0x8B5E => Ok(UniformType::Sampler2D),
            0x8B60 => Ok(UniformType::SamplerCube),
            _ => Err(anyhow::anyhow!("Unexpected uniform type: {}", value)),
        }
    }
}
