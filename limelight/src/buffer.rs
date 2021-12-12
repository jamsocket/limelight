use crate::{shadow_gpu::BufferHandle, webgl::buffer::BufferUsageHint, Attribute};
use std::marker::PhantomData;

#[allow(clippy::len_without_is_empty)]
pub trait BufferLike<T: Attribute> {
    fn get_buffer(&self) -> Option<BufferHandle>;

    fn len(&self) -> usize;
}

#[derive(Clone)]
pub struct Buffer<T: Attribute> {
    handle: BufferHandle,
    _ph: PhantomData<T>,
}

impl<T: Attribute> Buffer<T> {
    pub fn new(data: Vec<T>, usage_hint: BufferUsageHint) -> Self {
        let handle = BufferHandle::new(usage_hint);
        handle.set_data(data);

        Buffer {
            handle,
            _ph: PhantomData::default(),
        }
    }

    pub fn new_empty(usage_hint: BufferUsageHint) -> Self {
        Self::new(Vec::new(), usage_hint)
    }

    pub fn set_data(&self, data: Vec<T>) {
        self.handle.set_data(data);
    }
}

impl<T: Attribute> BufferLike<T> for Buffer<T> {
    fn get_buffer(&self) -> Option<BufferHandle> {
        Some(self.handle.clone())
    }

    fn len(&self) -> usize {
        self.handle.len()
    }
}

pub struct DummyBuffer {
    size: usize,
}

impl DummyBuffer {
    pub fn new(size: usize) -> Self {
        DummyBuffer { size }
    }
}

impl BufferLike<()> for DummyBuffer {
    fn len(&self) -> usize {
        self.size
    }

    fn get_buffer(&self) -> Option<BufferHandle> {
        None
    }
}
