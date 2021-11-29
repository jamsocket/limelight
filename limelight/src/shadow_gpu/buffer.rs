use crate::{shadow_gpu::types::{BufferBindPoint, BufferUsageHint}, types::SizedDataType};
use anyhow::{anyhow, Result};
use std::{cell::RefCell, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

struct BufferGlObjects {
    vao: WebGlVertexArrayObject,
    buffer: WebGlBuffer,
    capacity: usize,
}

pub struct BufferHandle {
    gl_objects: Rc<RefCell<Option<BufferGlObjects>>>,
    data: Option<Vec<u8>>,
    attributes: Vec<SizedDataType>,
}

impl PartialEq for BufferHandle {
    fn eq(&self, other: &Self) -> bool {
        if !Rc::ptr_eq(&self.gl_objects, &other.gl_objects) {
            return false;
        }

        self.data == other.data
    }
}

impl BufferHandle {
    pub fn new(attributes: &[SizedDataType]) -> Result<BufferHandle> {
        Ok(BufferHandle {
            gl_objects: Rc::new(RefCell::new(None)),
            data: None,
            attributes: attributes.iter().cloned().collect(),
        })
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

    pub fn bind(&mut self, gl: &WebGl2RenderingContext) -> Result<Self> {
        if let Some(data) = self.data.take() {
            let mut gl_objects = self.gl_objects.borrow_mut();

            if let Some(gl_objects) = &mut *gl_objects {
                if gl_objects.capacity > data.len() {
                    gl.buffer_sub_data_with_i32_and_u8_array(
                        BufferBindPoint::ArrayBuffer as _,
                        0,
                        &data,
                    );
                } else {
                    // The current buffer isn't big enough, need to discard it and create a new one.
                    gl.delete_buffer(Some(&gl_objects.buffer));
                    gl.delete_vertex_array(Some(&gl_objects.vao));

                    *gl_objects = Self::create(gl, &data, &self.attributes)?;
                }
            } else {
                // We have not created this buffer before.
                *gl_objects = Some(Self::create(gl, &data, &self.attributes)?);
            }
        } else {
            if let Some(gl_objects) = &*self.gl_objects.borrow() {
                
            }
        }
        todo!();
    }
}
