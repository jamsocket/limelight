use anyhow::anyhow;

#[derive(Clone, Copy, Debug, PartialEq)]
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


#[derive(Clone, Copy, Debug, PartialEq)]
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
