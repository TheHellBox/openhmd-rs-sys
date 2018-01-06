extern crate openhmd_rs_sys;
extern crate libc;
use openhmd_rs_sys::*;
use libc::{c_int, c_float};
use std::ffi::CStr;
fn main(){
    unsafe{
        let context = ohmd_ctx_create();
        let num_devices = ohmd_ctx_probe(context);
        println!("num devices: {}", num_devices);
        println!("");
        for i in 0 .. num_devices {
            let vendor = ohmd_list_gets(context, i, ohmd_string_value::OHMD_VENDOR);
            let product = ohmd_list_gets(context, i, ohmd_string_value::OHMD_PRODUCT);
            let path = ohmd_list_gets(context, i, ohmd_string_value::OHMD_PATH);
            println!("device {}", i);
            println!("  vendor:  {}", CStr::from_ptr(vendor).to_str().unwrap());
            println!("  product: {}", CStr::from_ptr(product).to_str().unwrap());
            println!("  path:    {}", CStr::from_ptr(path).to_str().unwrap());
            println!("");
        }

        println!("opening device: {}", 0);
        let device = ohmd_list_open_device(context, 0);
        let mut out: [c_int; 1] = [0];
        ohmd_device_geti(device, ohmd_int_value::OHMD_SCREEN_HORIZONTAL_RESOLUTION, &mut out);
        let width = out[0];
        ohmd_device_geti(device, ohmd_int_value::OHMD_SCREEN_VERTICAL_RESOLUTION, &mut out);
        let height = out[0];
        let mut out: [c_float; 16] = [0.0; 16];
        ohmd_device_getf(device, ohmd_float_value::OHMD_SCREEN_HORIZONTAL_SIZE, &mut out);
        let hsize = out[0];
        ohmd_device_getf(device, ohmd_float_value::OHMD_SCREEN_VERTICAL_SIZE, &mut out);
        let vsize = out[0];
        ohmd_device_getf(device, ohmd_float_value::OHMD_LENS_HORIZONTAL_SEPARATION, &mut out);
        let lens_separation = out[0];
        ohmd_device_getf(device, ohmd_float_value::OHMD_LENS_VERTICAL_POSITION, &mut out);
        let lens_vcenter = out[0];
        ohmd_device_getf(device, ohmd_float_value::OHMD_LEFT_EYE_FOV, &mut out);
        let left_eye_fov = out[0];
        ohmd_device_getf(device, ohmd_float_value::OHMD_RIGHT_EYE_FOV, &mut out);
        let right_eye_fov = out[0];
        ohmd_device_getf(device, ohmd_float_value::OHMD_LEFT_EYE_ASPECT_RATIO, &mut out);
        let left_eye_aspect = out[0];
        ohmd_device_getf(device, ohmd_float_value::OHMD_RIGHT_EYE_ASPECT_RATIO, &mut out);
        let right_eye_aspect = out[0];

        println!("resolution:              {} x {}", width, height);
        println!("hsize:                   {:.6}", hsize);
        println!("vsize:                   {:.6}", vsize);
        println!("lens separation:         {:.6}", lens_separation);
        println!("lens vcenter:            {:.6}", lens_vcenter);
        println!("left eye fov:            {:.6}", left_eye_fov);
        println!("right eye fov:           {:.6}", right_eye_fov);
        println!("left eye aspect:         {:.6}", left_eye_aspect);
        println!("right eye aspect:        {:.6}", right_eye_aspect);
        println!("");

        ohmd_ctx_update(context);
        let output = ohmd_device_getf(device, ohmd_float_value::OHMD_ROTATION_QUAT, &mut out);
        println!("{:?} {}", out, output);

        ohmd_close_device(device);
        ohmd_ctx_destroy(context);
    }
}
