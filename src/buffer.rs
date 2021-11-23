use std::marker::PhantomData;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};
use crate::vertex_attribute::VertexAttribute;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum BufferBindPoint {
    ArrayBuffer = 0x8892,
    ElementArrayBuffer = 0x8893,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum BufferUsageHint {
    StreamDraw = 0x88E0,
    StaticDraw = 0x88E4,
    DynamicDraw = 0x88E8,
}

pub struct AttributeBuffer<T: VertexAttribute> {
    inner: Option<WebGlBuffer>,
    capacity: i32, // Total capacity of GPU-side buffer, in elements of T.
    bind_point: BufferBindPoint,
    usage: BufferUsageHint,

    data: Vec<T>,
    dirty: bool,

    _phantom: PhantomData<T>,
}

impl<T: VertexAttribute> AttributeBuffer<T> {
    pub fn new(bind_point: BufferBindPoint, usage: BufferUsageHint) -> Self {
        Self {
            inner: None,
            capacity: 0,
            bind_point,
            usage,
            data: Vec::new(),
            dirty: true,
            _phantom: PhantomData::default(),
        }
    }

    pub fn bind(&mut self, gl: &WebGl2RenderingContext) {
        if self.dirty {
            if let Some(old_buffer) = self.inner.take() {
                gl.delete_buffer(Some(&old_buffer));
            }

            if self.data.len() * std::mem::size_of::<T>() > self.capacity as _ {
                // Data is too big for current buffer, we need to create a
                // new one.

                let inner = gl.create_buffer().unwrap();
                gl.bind_buffer(self.bind_point as _, Some(&inner));

                gl.buffer_data_with_u8_array(
                    self.bind_point as _,
                    bytemuck::cast_slice(&self.data),
                    self.usage as _,
                );

                self.capacity = self.data.len() as _;
            } else {
                // Data fits in the current buffer, we just overwrite it.

                gl.buffer_sub_data_with_i32_and_u8_array(
                    self.bind_point as _,
                    0,
                    bytemuck::cast_slice(&self.data),
                );
            }

            self.dirty = false;
        }
        gl.bind_buffer(self.bind_point as _, self.inner.as_ref());
    }

    pub fn set_data(&mut self, data: Vec<T>) {
        self.data = data;
        self.dirty = true;
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
