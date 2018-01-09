extern crate libc;
use libc::{c_char, c_int, c_float, c_void};
#[repr(C)]
pub struct ohmd_context {
    __private: c_void
}
#[repr(C)]
pub struct ohmd_device {
    __private: c_void
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_float_value{
    OHMD_ROTATION_QUAT = 1,
    OHMD_LEFT_EYE_GL_MODELVIEW_MATRIX = 2,
    OHMD_RIGHT_EYE_GL_MODELVIEW_MATRIX = 3,
    OHMD_LEFT_EYE_GL_PROJECTION_MATRIX = 4,
    OHMD_RIGHT_EYE_GL_PROJECTION_MATRIX = 5,
    OHMD_POSITION_VECTOR = 6,
    OHMD_SCREEN_HORIZONTAL_SIZE = 7,
    OHMD_SCREEN_VERTICAL_SIZE = 8,
    OHMD_LENS_HORIZONTAL_SEPARATION = 9,
    OHMD_LENS_VERTICAL_POSITION = 10,
    OHMD_LEFT_EYE_FOV = 11,
    OHMD_LEFT_EYE_ASPECT_RATIO = 12,
    OHMD_RIGHT_EYE_FOV = 13,
    OHMD_RIGHT_EYE_ASPECT_RATIO = 14,
    OHMD_EYE_IPD = 15,
    OHMD_PROJECTION_ZFAR = 16,
    OHMD_PROJECTION_ZNEAR = 17,
    OHMD_DISTORTION_K = 18
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_string_value{
    OHMD_VENDOR = 0,
    OHMD_PRODUCT = 1,
    OHMD_PATH = 2
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_int_value{
    OHMD_SCREEN_HORIZONTAL_RESOLUTION = 0,
    OHMD_SCREEN_VERTICAL_RESOLUTION = 1
}
#[link(name = "openhmd")]
extern {
    pub fn ohmd_ctx_create() -> &'static ohmd_context;
    pub fn ohmd_ctx_destroy(ctx: &ohmd_context);
    pub fn ohmd_ctx_get_error(ctx: &ohmd_context) -> *const c_char;
    pub fn ohmd_ctx_probe(ctx: &ohmd_context) -> c_int;
    pub fn ohmd_ctx_update(ctx: &ohmd_context);
    pub fn ohmd_device_getf(device: &ohmd_device, otype: ohmd_float_value, out: &mut [c_float; 16]) -> c_int;
    pub fn ohmd_device_setf(device: &ohmd_device, otype: ohmd_float_value, float: &[c_float; 16]) -> c_int;
    pub fn ohmd_list_open_device(ctx: &ohmd_context, index: c_int) -> &'static ohmd_device;
    pub fn ohmd_list_gets(ctx: &ohmd_context, index: c_int, otype: ohmd_string_value) -> *const c_char;
    pub fn ohmd_device_geti(device: &ohmd_device, otype: ohmd_int_value, out: &mut [c_int; 1]) -> c_int;
    pub fn ohmd_close_device(device: &ohmd_device) -> c_int;
}
