use std::rc::Rc;

use self::{blending::BlendFunction, culling::CullingMode, depth::DepthFunction};

pub mod blending;
pub mod enable;
pub mod culling;
pub mod depth;

#[derive(Default)]
pub struct State {
    pub blend_func: Option<BlendFunction>,
    pub culling: Option<CullingMode>,
    pub depth_func: Option<DepthFunction>,
}
