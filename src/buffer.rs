use crate::vertex_attribute::VertexAttribute;
use std::{cell::RefCell, fmt::Debug, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};
use crate::console_log;

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

impl<T: VertexAttribute> Debug for AttributeBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.inner.borrow();
        write!(
            f,
            "AttributeBuffer(bound={}, dirty={}, capacity={}, size={})",
            !inner.buffer.is_none(),
            inner.dirty,
            inner.capacity,
            inner.data.len()
        )
    }
}

pub trait BindableBuffer: Debug {
    fn bind(&self, gl: &WebGl2RenderingContext);

    fn len(&self) -> usize;

    fn boxed_clone(&self) -> Box<dyn BindableBuffer>;
}

impl<T: VertexAttribute> BindableBuffer for Rc<AttributeBuffer<T>> {
    fn boxed_clone(&self) -> Box<dyn BindableBuffer> {
        Box::new(self.clone())
    }

    fn len(&self) -> usize {
        self.inner.borrow().data.len()
    }

    fn bind(&self, gl: &WebGl2RenderingContext) {
        let mut inner = self.inner.borrow_mut();
        let bind_point = BufferBindPoint::ArrayBuffer;

        if inner.dirty {
            if inner.data.len() > inner.capacity as _ {
                if let Some(old_buffer) = inner.buffer.take() {
                    gl.delete_buffer(Some(&old_buffer));
                }

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
                inner.buffer = Some(buffer);
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
            }),
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
