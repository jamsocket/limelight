use crate::{
    shadow_gpu::types::{BufferBindPoint, BufferUsageHint},
    types::SizedDataType,
};
use anyhow::{anyhow, Result};
use std::{cell::RefCell, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

use super::GpuBind;

struct BufferGlObjects {
    vao: WebGlVertexArrayObject,
    buffer: WebGlBuffer,
    capacity: usize,
}

struct DataWithMarker {
    data: Vec<u8>,
    dirty: bool,
}

impl Default for DataWithMarker {
    fn default() -> Self {
        DataWithMarker {
            data: Vec::new(),
            dirty: true,
        }
    }
}

pub struct BufferHandleInner {
    gl_objects: RefCell<Option<BufferGlObjects>>,
    data: RefCell<DataWithMarker>,
    attributes: Vec<SizedDataType>,
}

#[derive(Clone)]
pub struct BufferHandle(Rc<BufferHandleInner>);

impl PartialEq for BufferHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl GpuBind for Option<BufferHandle> {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        if let Some(buffer) = self {
            buffer.bind(gl, false)?;
        } else {
            gl.bind_buffer(BufferBindPoint::ArrayBuffer as _, None);
        }

        Ok(())
    }
}

impl BufferHandle {
    pub fn new(attributes: &[SizedDataType]) -> BufferHandle {
        BufferHandle(Rc::new(BufferHandleInner {
            gl_objects: RefCell::new(None),
            data: RefCell::new(DataWithMarker::default()),
            attributes: attributes.iter().cloned().collect(),
        }))
    }

    fn create(
        gl: &WebGl2RenderingContext,
        data: &[u8],
        attributes: &[SizedDataType],
    ) -> Result<BufferGlObjects> {
        let vao = gl
            .create_vertex_array()
            .ok_or_else(|| anyhow!("Couldn't create vertex array."))?;

        gl.bind_vertex_array(Some(&vao));

        let buffer = gl
            .create_buffer()
            .ok_or_else(|| anyhow!("Couldn't create buffer."))?;

        gl.bind_buffer(BufferBindPoint::ArrayBuffer as _, Some(&buffer));
        gl.buffer_data_with_u8_array(
            BufferBindPoint::ArrayBuffer as _,
            &data,
            BufferUsageHint::DynamicDraw as _,
        );

        let mut offset: i32 = 0;
        let stride = attributes.iter().map(|d| d.byte_size()).sum();

        for (location, attr) in attributes.iter().enumerate() {
            gl.vertex_attrib_pointer_with_i32(
                location as _,
                attr.size(),
                attr.data_type() as _,
                false,
                stride,
                offset,
            );
            gl.enable_vertex_attrib_array(location as _);

            offset += attr.byte_size();
        }

        Ok(BufferGlObjects {
            buffer,
            vao,
            capacity: data.len(),
        })
    }

    pub fn sync_data(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        self.bind(gl, true)
    }

    pub fn bind(&self, gl: &WebGl2RenderingContext, already_bound: bool) -> Result<()> {
        let inner = &self.0;

        // The buffer handle has local data, so we need to write it.
        let mut gl_objects = inner.gl_objects.borrow_mut();
        let data = inner.data.borrow();

        if let Some(gl_objects) = &mut *gl_objects {
            if data.dirty {
                if gl_objects.capacity > data.data.len() {
                    if !already_bound {
                        gl.bind_vertex_array(Some(&gl_objects.vao));
                    }

                    gl.buffer_sub_data_with_i32_and_u8_array(
                        BufferBindPoint::ArrayBuffer as _,
                        0,
                        &data.data,
                    );
                } else {
                    // The current buffer isn't big enough, need to discard it and create a new one.
                    gl.delete_buffer(Some(&gl_objects.buffer));
                    gl.delete_vertex_array(Some(&gl_objects.vao));

                    *gl_objects = Self::create(gl, &data.data, &inner.attributes)?;
                }
            } else {
                if !already_bound {
                    gl.bind_vertex_array(Some(&gl_objects.vao));
                }
            }
        } else {
            // We have not created this buffer before.
            *gl_objects = Some(Self::create(gl, &data.data, &inner.attributes)?);
        }

        Ok(())
    }
}
