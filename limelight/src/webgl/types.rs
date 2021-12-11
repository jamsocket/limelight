use anyhow::anyhow;

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
#[repr(u32)]
pub enum DataType {
    Byte = 0x1400,
    UnsignedByte = 0x1401,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Int = 0x1404,
    UnsignedInt = 0x1405,
    Float = 0x1406,
}

impl TryFrom<u32> for DataType {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1400 => Ok(DataType::Byte),
            0x1401 => Ok(DataType::UnsignedByte),
            0x1402 => Ok(DataType::Short),
            0x1403 => Ok(DataType::UnsignedShort),
            0x1404 => Ok(DataType::Int),
            0x1405 => Ok(DataType::UnsignedInt),
            0x1406 => Ok(DataType::Float),
            _ => Err(anyhow!("Invalid DataType.")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct SizedDataType {
    data_type: DataType,
    size: i32,
}

impl SizedDataType {
    pub fn new(data_type: DataType, size: i32) -> Self {
        if !(1..=4).contains(&size) {
            panic!("Tried to create SizedDataType with size {} but glsl only supports vec{{2,3,4}} and scalars.", size);
        }

        SizedDataType { data_type, size }
    }

    pub fn byte_size(&self) -> i32 {
        self.data_type.size() * self.size
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn data_type(&self) -> DataType {
        self.data_type
    }
}

impl DataType {
    pub fn size(&self) -> i32 {
        match self {
            DataType::Byte | DataType::UnsignedByte => 1,
            DataType::Short | DataType::UnsignedShort => 2,
            DataType::Int | DataType::UnsignedInt => 4,
            DataType::Float => 4,
        }
    }
}

pub trait AsSizedDataType {
    fn as_sized_data_type() -> SizedDataType;
}

impl AsSizedDataType for f32 {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::Float,
            size: 1,
        }
    }
}

impl<const N: usize> AsSizedDataType for [f32; N] {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::Float,
            size: N as _,
        }
    }
}

impl AsSizedDataType for i32 {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::Int,
            size: 1,
        }
    }
}

impl<const N: usize> AsSizedDataType for [i32; N] {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::Int,
            size: N as _,
        }
    }
}

impl AsSizedDataType for u32 {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::UnsignedInt,
            size: 1,
        }
    }
}

impl<const N: usize> AsSizedDataType for [u32; N] {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::UnsignedInt,
            size: N as _,
        }
    }
}

impl AsSizedDataType for i16 {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::Short,
            size: 1,
        }
    }
}

impl<const N: usize> AsSizedDataType for [i16; N] {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::Short,
            size: N as _,
        }
    }
}

impl AsSizedDataType for u16 {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::UnsignedShort,
            size: 1,
        }
    }
}

impl<const N: usize> AsSizedDataType for [u16; N] {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::UnsignedShort,
            size: N as _,
        }
    }
}

impl AsSizedDataType for u8 {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::UnsignedByte,
            size: 1,
        }
    }
}

impl<const N: usize> AsSizedDataType for [u8; N] {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::UnsignedByte,
            size: N as _,
        }
    }
}

impl AsSizedDataType for i8 {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::Byte,
            size: 1,
        }
    }
}

impl<const N: usize> AsSizedDataType for [i8; N] {
    fn as_sized_data_type() -> SizedDataType {
        SizedDataType {
            data_type: DataType::Byte,
            size: N as _,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum GlSizedDataType {
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

impl GlSizedDataType {
    pub fn as_sized_type(&self) -> SizedDataType {
        match self {
            GlSizedDataType::Float => SizedDataType::new(DataType::Float, 1),
            GlSizedDataType::FloatVec2 => SizedDataType::new(DataType::Float, 2),
            GlSizedDataType::FloatVec3 => SizedDataType::new(DataType::Float, 3),
            GlSizedDataType::FloatVec4 => SizedDataType::new(DataType::Float, 4),
            GlSizedDataType::IntVec2 => todo!(),
            GlSizedDataType::IntVec3 => todo!(),
            GlSizedDataType::IntVec4 => todo!(),
            GlSizedDataType::Bool => todo!(),
            GlSizedDataType::BoolVec2 => todo!(),
            GlSizedDataType::BoolVec3 => todo!(),
            GlSizedDataType::BoolVec4 => todo!(),
            GlSizedDataType::FloatMat2 => todo!(),
            GlSizedDataType::FloatMat3 => todo!(),
            GlSizedDataType::FloatMat4 => todo!(),
            GlSizedDataType::Sampler2D => todo!(),
            GlSizedDataType::SamplerCube => todo!(),
            GlSizedDataType::Byte => todo!(),
            GlSizedDataType::UnsignedByte => todo!(),
            GlSizedDataType::Short => todo!(),
            GlSizedDataType::UnsignedShort => todo!(),
            GlSizedDataType::Int => SizedDataType::new(DataType::Int, 1),
            GlSizedDataType::UnsignedInt => SizedDataType::new(DataType::UnsignedInt, 1),
        }
    }
}

impl TryFrom<u32> for GlSizedDataType {
    type Error = anyhow::Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1400 => Ok(GlSizedDataType::Byte),
            0x1401 => Ok(GlSizedDataType::UnsignedByte),
            0x1402 => Ok(GlSizedDataType::Short),
            0x1403 => Ok(GlSizedDataType::UnsignedShort),
            0x1404 => Ok(GlSizedDataType::Int),
            0x1405 => Ok(GlSizedDataType::UnsignedInt),
            0x1406 => Ok(GlSizedDataType::Float),
            0x8B50 => Ok(GlSizedDataType::FloatVec2),
            0x8B51 => Ok(GlSizedDataType::FloatVec3),
            0x8B52 => Ok(GlSizedDataType::FloatVec4),
            0x8B53 => Ok(GlSizedDataType::IntVec2),
            0x8B54 => Ok(GlSizedDataType::IntVec3),
            0x8B55 => Ok(GlSizedDataType::IntVec4),
            0x8B56 => Ok(GlSizedDataType::Bool),
            0x8B57 => Ok(GlSizedDataType::BoolVec2),
            0x8B58 => Ok(GlSizedDataType::BoolVec3),
            0x8B59 => Ok(GlSizedDataType::BoolVec4),
            0x8B5A => Ok(GlSizedDataType::FloatMat2),
            0x8B5B => Ok(GlSizedDataType::FloatMat3),
            0x8B5C => Ok(GlSizedDataType::FloatMat4),
            0x8B5E => Ok(GlSizedDataType::Sampler2D),
            0x8B60 => Ok(GlSizedDataType::SamplerCube),
            _ => Err(anyhow::anyhow!("Unexpected uniform type: {}", value)),
        }
    }
}
