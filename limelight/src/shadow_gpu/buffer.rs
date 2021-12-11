use crate::webgl::buffer::{BufferBindPoint, BufferUsageHint};
use anyhow::{anyhow, Result};
use bytemuck::Pod;
use std::{cell::RefCell, hash::Hash, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub enum BindResult {
    BoundExisting,
    BoundNew,
}

struct BufferGlObjects {
    buffer: WebGlBuffer,
    capacity: usize,
}

trait AsBytes {
    fn as_bytes(&self) -> &[u8];

    fn byte_len(&self) -> usize;
}

impl<T: Pod> AsBytes for Vec<T> {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }

    fn byte_len(&self) -> usize {
        self.len() * std::mem::size_of::<T>() / std::mem::size_of::<u8>()
    }
}

struct DataWithMarker {
    data: Box<dyn AsBytes>,
    length: usize,
    dirty: bool,
}

impl Default for DataWithMarker {
    fn default() -> Self {
        DataWithMarker {
            data: Box::new(Vec::<u8>::new()),
            length: 0,
            dirty: true,
        }
    }
}

pub struct BufferHandleInner {
    gl_objects: RefCell<Option<BufferGlObjects>>,
    data: RefCell<DataWithMarker>,
    usage_hint: BufferUsageHint,
}

#[derive(Clone)]
pub struct BufferHandle(Rc<BufferHandleInner>);

impl PartialOrd for BufferHandle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for BufferHandle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let s = self.0.as_ref() as *const BufferHandleInner;
        let o = other.0.as_ref() as *const BufferHandleInner;

        s.cmp(&o)
    }
}

impl Hash for BufferHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // TODO: is Pin needed for correctness?
        (self.0.as_ref() as *const BufferHandleInner).hash(state)
    }
}

impl PartialEq for BufferHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for BufferHandle {}

impl BufferHandle {
    fn new_impl(usage_hint: BufferUsageHint) -> BufferHandle {
        BufferHandle(Rc::new(BufferHandleInner {
            gl_objects: RefCell::new(None),
            data: RefCell::new(DataWithMarker::default()),
            usage_hint,
        }))
    }

    pub fn new(usage_hint: BufferUsageHint) -> BufferHandle {
        Self::new_impl(usage_hint)
    }

    pub fn set_data<T: Pod>(&self, data: Vec<T>) {
        *self.0.data.borrow_mut() = DataWithMarker {
            length: data.len(),
            data: Box::new(data),
            dirty: true,
        };
    }

    pub fn len(&self) -> usize {
        self.0.data.borrow().length
    }

    fn create(
        gl: &WebGl2RenderingContext,
        data: &[u8],
        usage_hint: BufferUsageHint,
    ) -> Result<BufferGlObjects> {
        // let vao = gl
        //     .create_vertex_array()
        //     .ok_or_else(|| anyhow!("Couldn't create vertex array."))?;

        // gl.bind_vertex_array(Some(&vao));

        let buffer = gl
            .create_buffer()
            .ok_or_else(|| anyhow!("Couldn't create buffer."))?;

        gl.bind_buffer(BufferBindPoint::ArrayBuffer as _, Some(&buffer));
        gl.buffer_data_with_u8_array(BufferBindPoint::ArrayBuffer as _, &data, usage_hint as _);

        Ok(BufferGlObjects {
            buffer,
            capacity: data.len(),
        })
    }

    pub fn bind(&self, gl: &WebGl2RenderingContext) -> Result<BindResult> {
        let inner = &self.0;

        // The buffer handle has local data, so we need to write it.
        let mut gl_objects = inner.gl_objects.borrow_mut();
        let data = inner.data.borrow();

        if let Some(gl_objects) = &mut *gl_objects {
            if data.dirty {
                if gl_objects.capacity >= data.data.byte_len() {
                    gl.buffer_sub_data_with_i32_and_u8_array(
                        BufferBindPoint::ArrayBuffer as _,
                        0,
                        data.data.as_bytes(),
                    );
                    Ok(BindResult::BoundExisting)
                } else {
                    // The current buffer isn't big enough, need to discard it and create a new one.
                    gl.delete_buffer(Some(&gl_objects.buffer));

                    *gl_objects = Self::create(gl, data.data.as_bytes(), inner.usage_hint)?;
                    Ok(BindResult::BoundNew)
                }
            } else {
                gl.bind_buffer(BufferBindPoint::ArrayBuffer as _, Some(&gl_objects.buffer));
                Ok(BindResult::BoundExisting)
            }
        } else {
            // We have not created this buffer before.
            *gl_objects = Some(Self::create(gl, data.data.as_bytes(), inner.usage_hint)?);

            Ok(BindResult::BoundNew)
        }
    }
}
