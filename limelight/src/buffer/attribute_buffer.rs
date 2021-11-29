use super::{
    types::{BufferBindPoint, BufferUsageHint},
    BufferLike,
};
use crate::{VertexAttribute, VertexAttributeBinding, shadow_gpu::{BufferHandle, ShadowGpu}};
use anyhow::{anyhow, Result};
use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

struct BoundBuffer {
    buffer: WebGlBuffer,
    vao: Rc<WebGlVertexArrayObject>,
    capacity: u32,
    dirty: bool,
}

/// Represents a buffer used to store vertex attributes of a given type.
pub struct AttributeBuffer<T: VertexAttribute> {
    bound_buffer: RefCell<Option<BoundBuffer>>,
    usage: BufferUsageHint,
    data: RefCell<Vec<T>>,
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
            self.data.borrow().len()
        )
    }
}

const BIND_POINT: BufferBindPoint = BufferBindPoint::ArrayBuffer;

impl<T: VertexAttribute> BufferLike<T> for AttributeBuffer<T> {
    fn len(&self) -> usize {
        self.data.borrow().len()
    }

    fn get_vao(&self, gpu: &ShadowGpu) -> Result<BufferHandle> {
        let mut buffer_ref = self.bound_buffer.borrow_mut();
        let vao = if let Some(BoundBuffer {
            buffer,
            vao,
            capacity,
            dirty,
        }) = &mut *buffer_ref
        {
            if !*dirty {
                // Bind buffer.
                gl.bind_vertex_array(Some(vao));

                vao.clone()
            } else if self.data.borrow().len() > *capacity as _ {
                // Resize buffer.

                gl.delete_buffer(Some(&buffer));
                gl.delete_vertex_array(Some(&vao));

                *dirty = false;
                self.create_and_bind_buffer(gl, buffer_ref)?
            } else {
                // Reuse buffer.
                gl.bind_vertex_array(Some(&vao));

                gl.buffer_sub_data_with_i32_and_u8_array(
                    BIND_POINT as _,
                    0,
                    bytemuck::cast_slice(&self.data.borrow()),
                );

                *dirty = false;

                vao.clone()
            }
        } else {
            // Create buffer.
            self.create_and_bind_buffer(gl, buffer_ref)?
        };

        Ok(Some(vao))
    }
}

impl<T: VertexAttribute> AttributeBuffer<T> {
    fn create_and_bind_buffer(
        &self,
        gl: &WebGl2RenderingContext,
        mut buffer_ref: RefMut<Option<BoundBuffer>>,
    ) -> Result<Rc<WebGlVertexArrayObject>> {
        let vao = gl
            .create_vertex_array()
            .ok_or_else(|| anyhow!("Couldn't create vertex array."))?;
        gl.bind_vertex_array(Some(&vao));

        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(BIND_POINT as _, Some(&buffer));

        gl.buffer_data_with_u8_array(
            BIND_POINT as _,
            bytemuck::cast_slice(&self.data.borrow()),
            self.usage as _,
        );

        bind_vertex_attributes(&T::describe(), &gl);

        let vao = Rc::new(vao);
        *buffer_ref = Some(BoundBuffer {
            vao: vao.clone(),
            buffer,
            capacity: self.data.borrow().len() as _,
            dirty: false,
        });

        Ok(vao)
    }

    pub fn new(usage: BufferUsageHint) -> Self {
        AttributeBuffer {
            bound_buffer: RefCell::new(None),
            usage,
            data: RefCell::new(Vec::new()),
        }
    }

    pub fn set_data(&mut self, data: Vec<T>) {
        *self.data.borrow_mut() = data;
        if let Some(buffer) = &mut *self.bound_buffer.borrow_mut() {
            buffer.dirty = true;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
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
