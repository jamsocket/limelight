use crate::{
    shadow_gpu::types::{BufferBindPoint, BufferUsageHint},
    types::SizedDataType,
};
use anyhow::{anyhow, Result};
use bytemuck::Pod;
use std::{cell::RefCell, rc::Rc};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

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
    attributes: Vec<SizedDataType>,
    usage_hint: BufferUsageHint,
    instance_attribute: bool,
}

#[derive(Clone)]
pub struct BufferHandle(Rc<BufferHandleInner>);

impl PartialEq for BufferHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

// impl GpuBind for Option<BufferHandle> {
//     fn gpu_bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
//         if let Some(buffer) = self {
//             buffer.bind(gl, false)?;
//         } else {
//             gl.bind_buffer(BufferBindPoint::ArrayBuffer as _, None);
//         }

//         Ok(())
//     }
// }

impl BufferHandle {
    fn new_impl(
        usage_hint: BufferUsageHint,
        attributes: &[SizedDataType],
        instance_attribute: bool,
    ) -> BufferHandle {
        BufferHandle(Rc::new(BufferHandleInner {
            gl_objects: RefCell::new(None),
            data: RefCell::new(DataWithMarker::default()),
            attributes: attributes.iter().cloned().collect(),
            usage_hint,
            instance_attribute,
        }))
    }

    pub fn new(usage_hint: BufferUsageHint, attributes: &[SizedDataType]) -> BufferHandle {
        Self::new_impl(usage_hint, attributes, false)
    }

    pub fn new_instanced(
        usage_hint: BufferUsageHint,
        attributes: &[SizedDataType],
    ) -> BufferHandle {
        Self::new_impl(usage_hint, attributes, false)
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
        attributes: &[SizedDataType],
        usage_hint: BufferUsageHint,
        instanced: bool,
    ) -> Result<BufferGlObjects> {
        let vao = gl
            .create_vertex_array()
            .ok_or_else(|| anyhow!("Couldn't create vertex array."))?;

        gl.bind_vertex_array(Some(&vao));

        let buffer = gl
            .create_buffer()
            .ok_or_else(|| anyhow!("Couldn't create buffer."))?;

        gl.bind_buffer(BufferBindPoint::ArrayBuffer as _, Some(&buffer));
        gl.buffer_data_with_u8_array(BufferBindPoint::ArrayBuffer as _, &data, usage_hint as _);

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

            if instanced {
                gl.vertex_attrib_divisor(location as _, 1);
            }

            gl.enable_vertex_attrib_array(location as _);
            offset += attr.byte_size();
        }

        Ok(BufferGlObjects {
            buffer,
            capacity: data.len(),
        })
    }

    pub fn bind(&self, gl: &WebGl2RenderingContext) -> Result<()> {
        let inner = &self.0;

        // The buffer handle has local data, so we need to write it.
        let mut gl_objects = inner.gl_objects.borrow_mut();
        let data = inner.data.borrow();

        if let Some(gl_objects) = &mut *gl_objects {
            if data.dirty {
                if gl_objects.capacity > data.data.byte_len() {
                    gl.buffer_sub_data_with_i32_and_u8_array(
                        BufferBindPoint::ArrayBuffer as _,
                        0,
                        data.data.as_bytes(),
                    );
                } else {
                    // The current buffer isn't big enough, need to discard it and create a new one.
                    gl.delete_buffer(Some(&gl_objects.buffer));

                    *gl_objects = Self::create(
                        gl,
                        data.data.as_bytes(),
                        &inner.attributes,
                        inner.usage_hint,
                        inner.instance_attribute,
                    )?;
                }
            } else {

            }
        } else {
            // We have not created this buffer before.
            *gl_objects = Some(Self::create(
                gl,
                data.data.as_bytes(),
                &inner.attributes,
                inner.usage_hint,
                inner.instance_attribute,
            )?);
        }

        Ok(())
    }
}
