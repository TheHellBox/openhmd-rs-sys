extern crate libc;
use libc::{c_char, c_int, c_float};
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ohmd_context;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ohmd_device;
#[derive(Clone, Copy)]
pub enum ohmd_float_value{
    OHMD_ROTATION_QUAT,
    OHMD_LEFT_EYE_GL_MODELVIEW_MATRIX,
    OHMD_RIGHT_EYE_GL_MODELVIEW_MATRIX,
    OHMD_LEFT_EYE_GL_PROJECTION_MATRIX,
    OHMD_RIGHT_EYE_GL_PROJECTION_MATRIX,
    OHMD_POSITION_VECTOR,
    OHMD_SCREEN_HORIZONTAL_SIZE,
    OHMD_SCREEN_VERTICAL_SIZE,
    OHMD_LENS_HORIZONTAL_SEPARATION,
    OHMD_LENS_VERTICAL_POSITION,
    OHMD_LEFT_EYE_FOV,
    OHMD_LEFT_EYE_ASPECT_RATIO,
    OHMD_RIGHT_EYE_FOV,
    OHMD_RIGHT_EYE_ASPECT_RATIO,
    OHMD_EYE_IPD,
    OHMD_PROJECTION_ZFAR,
    OHMD_PROJECTION_ZNEAR,
    OHMD_DISTORTION_K
}
#[derive(Clone, Copy)]
pub enum ohmd_string_value{
    OHMD_VENDOR = 0,
    OHMD_PRODUCT = 1,
    OHMD_PATH = 2
}
#[derive(Clone, Copy)]
pub enum ohmd_int_value{
    OHMD_SCREEN_HORIZONTAL_RESOLUTION,
    OHMD_SCREEN_VERTICAL_RESOLUTION
}
#[link(name = "openhmd")]
extern {
    pub fn ohmd_ctx_create() -> &'static ohmd_context;
    pub fn ohmd_ctx_destroy(ctx: &ohmd_context) -> ohmd_context;
    pub fn ohmd_ctx_get_error(ctx: &ohmd_context) -> c_char;
    pub fn ohmd_ctx_probe(ctx: &ohmd_context) -> c_int;
    pub fn ohmd_ctx_update(ctx: &ohmd_context);
    pub fn ohmd_device_getf(device: &ohmd_device, otype: ohmd_float_value, out: c_float) -> c_int;
    pub fn ohmd_device_setf(device: &ohmd_device, otype: ohmd_float_value, float: c_float) -> c_int;
    pub fn ohmd_list_open_device(ctx: &ohmd_context, index: c_int) -> &'static ohmd_device;
    pub fn ohmd_list_gets(ctx: &ohmd_context, index: c_int, otype: ohmd_string_value) -> c_char;
    pub fn ohmd_device_geti(device: &ohmd_device, otype: ohmd_int_value, out: c_int) -> c_int;
}
