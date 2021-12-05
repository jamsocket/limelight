use crate::{shadow_gpu::{BufferHandle, BufferUsageHint}, types::SizedDataType, Attribute};
use std::marker::PhantomData;

pub trait AttribDivisor {
    fn attrib_divisor(&self) -> Option<usize>;
}

pub struct VertexAttribute;

impl AttribDivisor for VertexAttribute {
    fn attrib_divisor(&self) -> Option<usize> {
        None
    }
}

pub struct InstanceAttribute;

impl AttribDivisor for InstanceAttribute {
    fn attrib_divisor(&self) -> Option<usize> {
        Some(1)
    }
}

pub trait BufferLike<T: Attribute, A: AttribDivisor> {
    fn get_buffer(&self) -> Option<BufferHandle>;

    fn len(&self) -> usize;
}

pub struct Buffer<T: Attribute, A: AttribDivisor> {
    handle: BufferHandle,
    _ph: PhantomData<T>,
    _pha: PhantomData<A>,
}

impl<T: Attribute, A: AttribDivisor> Buffer<T, A> {
    pub fn new(data: Vec<T>, usage_hint: BufferUsageHint) -> Self {
        let attributes: Vec<SizedDataType> = T::describe().into_iter().map(|d| d.kind).collect();
        let handle = BufferHandle::new(usage_hint, &attributes);

        handle.set_data(data);

        Buffer {
            handle,
            _ph: PhantomData::default(),
            _pha: PhantomData::default(),
        }
    }

    pub fn set_data(&self, data: Vec<T>) {
        self.handle.set_data(data);
    }
}

impl<T: Attribute, A: AttribDivisor> BufferLike<T, A> for Buffer<T, A> {
    fn get_buffer(&self) -> Option<BufferHandle> {
        Some(self.handle.clone())
    }

    fn len(&self) -> usize {
        self.handle.len()
    }
}

pub struct DummyBuffer<A: AttribDivisor> {
    size: usize,
    _ph: PhantomData<A>,
}

impl<A: AttribDivisor> DummyBuffer<A> {
    pub fn new(size: usize) -> Self {
        DummyBuffer {
            size,
            _ph: PhantomData::default(),
        }
    }
}

impl<A: AttribDivisor> BufferLike<(), A> for DummyBuffer<A> {
    fn len(&self) -> usize {
        self.size
    }

    fn get_buffer(&self) -> Option<BufferHandle> {
        None
    }
}