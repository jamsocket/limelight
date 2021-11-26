use crate::{
    gpu_init::GpuBind,
    vertex_attribute::{VertexAttribute, VertexAttributeBinding},
};
use anyhow::{anyhow, Result};
use std::{cell::RefCell, fmt::Debug};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

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

struct BoundBuffer {
    buffer: WebGlBuffer,
    vao: WebGlVertexArrayObject,
    capacity: u32,
}

pub struct AttributeBuffer<T: VertexAttribute> {
    bound_buffer: RefCell<Option<BoundBuffer>>,
    usage: BufferUsageHint,
    data: Vec<T>,
    dirty: bool,
}

impl<T: VertexAttribute> Debug for AttributeBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bound_buffer = self.bound_buffer.borrow();

        write!(
            f,
            "AttributeBuffer(bound={}, dirty={}, capacity={}, size={})",
            !bound_buffer.is_none(),
            self.dirty,
            bound_buffer
                .as_ref()
                .map(|d| d.capacity)
                .unwrap_or_default(),
            self.data.len()
        )
    }
}

const BIND_POINT: BufferBindPoint = BufferBindPoint::ArrayBuffer;

impl<T: VertexAttribute> GpuBind for AttributeBuffer<T> {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        let buffer_ref = self.bound_buffer.borrow();
        if let Some(BoundBuffer {
            buffer,
            vao,
            capacity,
        }) = &*buffer_ref
        {
            if !self.dirty {
                // Bind buffer.
                gl.bind_vertex_array(Some(vao));
            } else if self.data.len() > *capacity as _ {
                // Resize buffer.

                gl.delete_buffer(Some(&buffer));
                gl.delete_vertex_array(Some(&vao));

                drop(buffer_ref); // Free the borrow, because we need a mutable borrow.
                self.create_and_bind_buffer(gl)?;
            } else {
                // Reuse buffer.
                gl.bind_vertex_array(Some(&vao));

                gl.buffer_sub_data_with_i32_and_u8_array(
                    BIND_POINT as _,
                    0,
                    bytemuck::cast_slice(&self.data),
                );
            }
        } else {
            // Create buffer.
            drop(buffer_ref); // Free the borrow, because we need a mutable borrow.
            self.create_and_bind_buffer(gl)?;
        }

        Ok(())
    }
}

impl<T: VertexAttribute> AttributeBuffer<T> {
    fn create_and_bind_buffer(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        let vao = gl
            .create_vertex_array()
            .ok_or_else(|| anyhow!("Couldn't create vertex array."))?;
        gl.bind_vertex_array(Some(&vao));

        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(BIND_POINT as _, Some(&buffer));

        gl.buffer_data_with_u8_array(
            BIND_POINT as _,
            bytemuck::cast_slice(&self.data),
            self.usage as _,
        );

        bind_vertex_attributes(&T::describe(), &gl);

        self.bound_buffer.replace(Some(BoundBuffer {
            vao,
            buffer,
            capacity: self.data.len() as _,
        }));

        Ok(())
    }

    pub fn new(usage: BufferUsageHint) -> Self {
        AttributeBuffer {
            bound_buffer: RefCell::new(None),
            usage,
            data: Vec::new(),
            dirty: true,
        }
    }

    pub fn set_data(&mut self, data: Vec<T>) {
        self.data = data;
        self.dirty = true;
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub fn bind_vertex_attributes(bindings: &[VertexAttributeBinding], gl: &WebGl2RenderingContext) {
    let mut offset: i32 = 0;
    let stride = bindings.iter().map(|d| d.kind.byte_size()).sum();

    for (location, binding) in bindings.iter().enumerate() {
        gl.vertex_attrib_pointer_with_i32(
            location as _,
            binding.kind.size(),
            binding.kind.data_type() as _,
            false,
            stride,
            offset,
        );
        gl.enable_vertex_attrib_array(location as _);

        offset += binding.kind.byte_size();
    }
}
