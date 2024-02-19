use crate::*;

pub type ClientMode = WithVmt<VMTClientMode>;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VMTClientMode {
    _pad1: [cfn!(c_void,c_void); 17],
    pub OverrideView: cfn!(c_void, *mut ClientMode,*mut ViewSetup),
    _pad2: [cfn!(c_void,c_void); 4],
    pub CreateMove: cfn!(bool, *mut ClientMode, c_float,*mut UserCmd),
}