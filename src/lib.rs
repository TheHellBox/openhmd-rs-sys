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
    OHMD_DISTORTION_K = 18,
    OHMD_EXTERNAL_SENSOR_FUSION = 19,
    OHMD_UNIVERSAL_DISTORTION_K = 20,
    OHMD_UNIVERSAL_ABERRATION_K = 21,
    OHMD_CONTROLS_STATE = 22
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_status{
    OHMD_S_OK = 0,
    OHMD_S_UNKNOWN_ERROR = -1,
    OHMD_S_INVALID_PARAMETER = -2,
    OHMD_S_UNSUPPORTED = -3,
    OHMD_S_INVALID_OPERATION = -4,
    OHMD_S_USER_RESERVED = -16384
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
pub enum ohmd_string_description{
    OHMD_GLSL_DISTORTION_VERT_SRC = 0,
    OHMD_GLSL_DISTORTION_FRAG_SRC = 1,
    OHMD_GLSL_330_DISTORTION_VERT_SRC = 2,
    OHMD_GLSL_330_DISTORTION_FRAG_SRC = 3,
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_control_hint{
    OHMD_GENERIC = 0,
    OHMD_TRIGGER = 1,
    OHMD_TRIGGER_CLICK = 2,
    OHMD_SQUEEZE = 3,
    OHMD_MENU = 4,
    OHMD_HOME = 5,
    OHMD_ANALOG_X = 6,
    OHMD_ANALOG_Y = 7,
    OHMD_ANALOG_PRESS = 8,
    OHMD_BUTTON_A = 9,
    OHMD_BUTTON_B = 10,
    OHMD_BUTTON_X = 11,
    OHMD_BUTTON_Y = 12
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_control_type{
    OHMD_DIGITAL = 0,
    OHMD_ANALOG = 1
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_int_value{
    OHMD_SCREEN_HORIZONTAL_RESOLUTION = 0,
    OHMD_SCREEN_VERTICAL_RESOLUTION = 1,
    OHMD_DEVICE_CLASS = 2,
    OHMD_DEVICE_FLAGS = 3,
    OHMD_CONTROL_COUNT = 4,
    OHMD_CONTROLS_HINTS = 5,
    OHMD_CONTROLS_TYPES = 6
}

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum ohmd_device_class{
    OHMD_DEVICE_CLASS_HMD = 0,
    OHMD_DEVICE_CLASS_CONTROLLER = 1,
    OHMD_DEVICE_CLASS_GENERIC_TRACKER = 2,
}

pub const OHMD_DEVICE_FLAGS_NULL_DEVICE: c_int = 1;
pub const OHMD_DEVICE_FLAGS_POSITIONAL_TRACKING: c_int = 2;
pub const OHMD_DEVICE_FLAGS_ROTATIONAL_TRACKING: c_int = 4;
pub const OHMD_DEVICE_FLAGS_LEFT_CONTROLLER: c_int = 8;
pub const OHMD_DEVICE_FLAGS_RIGHT_CONTROLLER: c_int = 16;

#[link(name = "openhmd")]
extern {
    pub fn ohmd_ctx_create() -> &'static ohmd_context;
    pub fn ohmd_ctx_destroy(ctx: &ohmd_context);
    pub fn ohmd_ctx_get_error(ctx: &ohmd_context) -> *const c_char;
    pub fn ohmd_ctx_probe(ctx: &ohmd_context) -> c_int;
    pub fn ohmd_ctx_update(ctx: &ohmd_context);
    pub fn ohmd_gets(stype: ohmd_string_description, out: &mut *const c_char) -> c_int;
    pub fn ohmd_device_getf(device: &ohmd_device, otype: ohmd_float_value, out: &mut [c_float; 16]) -> c_int;
    pub fn ohmd_device_setf(device: &ohmd_device, otype: ohmd_float_value, float: &[c_float; 16]) -> c_int;
    pub fn ohmd_list_open_device(ctx: &ohmd_context, index: c_int) -> &'static ohmd_device;
    pub fn ohmd_list_gets(ctx: &ohmd_context, index: c_int, otype: ohmd_string_value) -> *const c_char;
    pub fn ohmd_list_geti(ctx: &ohmd_context, index: c_int, otype: ohmd_int_value, out: &mut [c_int; 1]) -> c_int;
    pub fn ohmd_device_geti(device: &ohmd_device, otype: ohmd_int_value, out: &mut [c_int; 1]) -> c_int;
    pub fn ohmd_close_device(device: &ohmd_device) -> c_int;
}
