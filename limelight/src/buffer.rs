use crate::{
    gpu_init::GpuBind,
    vertex_attribute::{VertexAttribute, VertexAttributeBinding},
};
use anyhow::{anyhow, Result};
use std::{cell::{RefCell, RefMut}, fmt::Debug};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

pub trait BufferLike<T: VertexAttribute>: GpuBind {
    fn len(&self) -> usize;
}

/// Enum of WebGL Bind Points.
/// 
/// Each bind point is a global bind point in WebGL that can have an
/// array bound to it.
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum BufferBindPoint {
    ArrayBuffer = 0x8892,
    ElementArrayBuffer = 0x8893,
}

/// Usage hint to tell WebGL how a buffer will be used.
/// 
/// These hints are non-binding; you can read/write from a
/// buffer as much as you like regardless of the hint. However,
/// a driver may use the hint to optimize memory layout.
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum BufferUsageHint {
    /// Hint that a buffer is written to once and read once.
    StreamDraw = 0x88E0,

    /// Hint that a buffer is written to once and ready many times.
    StaticDraw = 0x88E4,

    /// Hint that a buffer is written and read many times.
    DynamicDraw = 0x88E8,
}

struct BoundBuffer {
    buffer: WebGlBuffer,
    vao: WebGlVertexArrayObject,
    capacity: u32,
    dirty: bool,
}

/// Represents a buffer used to store vertex attributes of a given type.
pub struct AttributeBuffer<T: VertexAttribute> {
    bound_buffer: RefCell<Option<BoundBuffer>>,
    usage: BufferUsageHint,
    data: Vec<T>,
}

impl<T: VertexAttribute> Debug for AttributeBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bound_buffer = self.bound_buffer.borrow();

        write!(
            f,
            "AttributeBuffer(bound={}, dirty={}, capacity={}, size={})",
            !bound_buffer.is_none(),
            bound_buffer.as_ref().map(|d| d.dirty).unwrap_or_default(),
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
        let mut buffer_ref = self.bound_buffer.borrow_mut();
        if let Some(BoundBuffer {
            buffer,
            vao,
            capacity,
            dirty,
        }) = &mut*buffer_ref
        {
            if !*dirty {
                // Bind buffer.
                gl.bind_vertex_array(Some(vao));
            } else if self.data.len() > *capacity as _ {
                // Resize buffer.

                gl.delete_buffer(Some(&buffer));
                gl.delete_vertex_array(Some(&vao));

                *dirty = false;
                self.create_and_bind_buffer(gl, buffer_ref)?;
            } else {
                // Reuse buffer.
                gl.bind_vertex_array(Some(&vao));

                gl.buffer_sub_data_with_i32_and_u8_array(
                    BIND_POINT as _,
                    0,
                    bytemuck::cast_slice(&self.data),
                );

                *dirty = false;
            }
        } else {
            // Create buffer.
            self.create_and_bind_buffer(gl, buffer_ref)?;
        }

        Ok(())
    }
}

impl<T: VertexAttribute> BufferLike<T> for AttributeBuffer<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<T: VertexAttribute> AttributeBuffer<T> {
    fn create_and_bind_buffer(&self, gl: &WebGl2RenderingContext, mut buffer_ref: RefMut<Option<BoundBuffer>>) -> Result<()> {
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

        *buffer_ref = Some(BoundBuffer {
            vao,
            buffer,
            capacity: self.data.len() as _,
            dirty: false,
        });

        Ok(())
    }

    pub fn new(usage: BufferUsageHint) -> Self {
        AttributeBuffer {
            bound_buffer: RefCell::new(None),
            usage,
            data: Vec::new(),
        }
    }

    pub fn set_data(&mut self, data: Vec<T>) {
        self.data = data;
        if let Some(buffer) = &mut*self.bound_buffer.borrow_mut() {
            buffer.dirty = true;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn bind_vertex_attributes(bindings: &[VertexAttributeBinding], gl: &WebGl2RenderingContext) {
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

pub struct DummyBuffer {
    size: usize
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
}

impl GpuBind for DummyBuffer {
    fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        gl.bind_vertex_array(None);
        Ok(())
    }
}


impl VertexAttribute for () {
    fn describe() -> Vec<VertexAttributeBinding> { vec![] }
}