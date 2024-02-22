use crate::*;

pub type RenderView = WithVmt<VMTRenderView>;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FloatRGBA(f32, f32, f32, f32);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTRenderView {
    _pad1: [u32; 4],
    pub set_blend: cfn!(c_void, &'static RenderView, f32),
    pub get_blend: cfn!(f32, &'static RenderView),
    pub set_color_modulation: cfn!(c_void, &'static RenderView, &'static FloatRGBA),
    pub get_color_modulation: cfn!(c_void, &'static RenderView, &'static mut FloatRGBA),
    _pad2: [u32; 42],
    pub get_matrices_for_view: cfn!(
        c_void,
        &'static RenderView,
        &'static VMatrix,
        &'static VMatrix,
        &'static VMatrix,
        &'static VMatrix
    ),
}
