use std::{cell::RefCell, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};
use crate::vertex_attribute::{VertexAttribute};

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

struct AttributeBufferInner<T: VertexAttribute> {
    buffer: Option<WebGlBuffer>,
    capacity: i32, // Total capacity of GPU-side buffer, in elements of T.
    usage: BufferUsageHint,

    data: Vec<T>,
    dirty: bool,
}

pub struct AttributeBuffer<T: VertexAttribute> {
    inner: RefCell<AttributeBufferInner<T>>,
}

pub trait BindableBuffer {
    fn bind(&self, gl: &WebGl2RenderingContext);

    fn len(&self) -> usize;
}

impl<T: VertexAttribute> BindableBuffer for AttributeBuffer<T> {
    fn len(&self) -> usize {
        self.inner.borrow().data.len()
    }

    fn bind(&self, gl: &WebGl2RenderingContext) {
        let mut inner = self.inner.borrow_mut();
        let bind_point = BufferBindPoint::ArrayBuffer;

        if inner.dirty {
            if let Some(old_buffer) = inner.buffer.take() {
                gl.delete_buffer(Some(&old_buffer));
            }

            if inner.data.len() * std::mem::size_of::<T>() > inner.capacity as _ {
                // Data is too big for current buffer, we need to create a
                // new one.

                let buffer = gl.create_buffer().unwrap();
                gl.bind_buffer(bind_point as _, Some(&buffer));

                gl.buffer_data_with_u8_array(
                    bind_point as _,
                    bytemuck::cast_slice(&inner.data),
                    inner.usage as _,
                );

                inner.capacity = inner.data.len() as _;
            } else {
                // Data fits in the current buffer, we just overwrite it.
                gl.bind_buffer(bind_point as _, inner.buffer.as_ref());

                gl.buffer_sub_data_with_i32_and_u8_array(
                    bind_point as _,
                    0,
                    bytemuck::cast_slice(&inner.data),
                );
            }

            inner.dirty = false;
        } else {
            gl.bind_buffer(bind_point as _, inner.buffer.as_ref());
        }
    }
}

impl<T: VertexAttribute> AttributeBuffer<T> {
    pub fn new(usage: BufferUsageHint) -> Rc<Self> {
        Rc::new(Self {
            inner: RefCell::new(AttributeBufferInner {
                buffer: None,
                capacity: 0,
                usage,
                data: Vec::new(),
                dirty: true,   
            })
        })
    }

    pub fn set_data(&self, data: Vec<T>) {
        let mut inner = self.inner.borrow_mut();
        *&mut inner.data = data;
        *&mut inner.dirty = true;
    }

    pub fn is_empty(&self) -> bool {
        self.inner.borrow().data.is_empty()
    }
}
