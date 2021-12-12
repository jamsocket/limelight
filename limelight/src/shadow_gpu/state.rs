use crate::{state::StateDescriptor, webgl::types::SizedDataType};
use std::collections::{BTreeMap, HashMap};

use super::{program::ProgramHandle, BufferHandle, UniformHandle, UniformValue};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct BufferBinding {
    // pub name: String,
    pub kind: SizedDataType,
    pub location: u32,
    pub normalized: bool,
    pub stride: i32,
    pub offset: i32,
    pub divisor: u32,
    //pub buffer: BufferHandle,
}

#[derive(Default)]
pub struct GpuState {
    pub program: Option<ProgramHandle>,
    pub buffers: BTreeMap<BufferHandle, Vec<BufferBinding>>,
    pub uniforms: HashMap<UniformHandle, UniformValue>,
    pub globals: StateDescriptor,
}
