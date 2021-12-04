use crate::{shadow_gpu::{BufferHandle, BufferUsageHint}, types::SizedDataType, VertexAttribute};
use std::marker::PhantomData;

pub trait BufferLike<T: VertexAttribute> {
    fn get_buffer(&self) -> Option<BufferHandle>;

    fn len(&self) -> usize;
}

pub struct Buffer<T: VertexAttribute> {
    handle: BufferHandle,
    _ph: PhantomData<T>,
}

impl<T: VertexAttribute> Buffer<T> {
    pub fn new(data: Vec<T>, usage_hint: BufferUsageHint) -> Self {
        let attributes: Vec<SizedDataType> = T::describe().into_iter().map(|d| d.kind).collect();
        let handle = BufferHandle::new(usage_hint, &attributes);

        handle.set_data(data);

        Buffer {
            handle,
            _ph: PhantomData::default(),
        }
    }

    pub fn set_data(&self, data: Vec<T>) {
        self.handle.set_data(data);
    }
}

impl<T: VertexAttribute> BufferLike<T> for Buffer<T> {
    fn get_buffer(&self) -> Option<BufferHandle> {
        Some(self.handle.clone())
    }

    fn len(&self) -> usize {
        self.handle.len()
    }
}

pub struct DummyBuffer(usize);

impl DummyBuffer {
    pub fn new(size: usize) -> Self {
        DummyBuffer(size)
    }
}

impl BufferLike<()> for DummyBuffer {
    fn len(&self) -> usize {
        self.0
    }

    fn get_buffer(&self) -> Option<BufferHandle> {
        None
    }
}