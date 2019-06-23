extern crate libc;
extern crate openhmd_rs_sys;
use libc::{c_float, c_int};
use openhmd_rs_sys::*;
use std::ffi::CStr;
use std::time::Duration;
fn main() {
    unsafe {
        let context = ohmd_ctx_create();
        let num_devices = ohmd_ctx_probe(context);
        println!("num devices: {}", num_devices);
        println!("");
        for i in 0..num_devices {
            let vendor = ohmd_list_gets(context, i, ohmd_string_value::OHMD_VENDOR);
            let product = ohmd_list_gets(context, i, ohmd_string_value::OHMD_PRODUCT);
            let path = ohmd_list_gets(context, i, ohmd_string_value::OHMD_PATH);
            let mut out: [c_int; 1] = [0; 1];
            ohmd_list_geti(context, i, ohmd_int_value::OHMD_DEVICE_CLASS, &mut out);
            let device_class_s = ["HMD", "Controller", "Generic Tracker", "Unknown"];
            let class = out[0] as usize;
            ohmd_list_geti(context, i, ohmd_int_value::OHMD_DEVICE_FLAGS, &mut out);
            let flags = out[0];
            println!("device {}", i);
            println!("  vendor:  {}", CStr::from_ptr(vendor).to_str().unwrap());
            println!("  product: {}", CStr::from_ptr(product).to_str().unwrap());
            println!("  path:    {}", CStr::from_ptr(path).to_str().unwrap());
            println!(
                "  class:   {}",
                device_class_s[if class > 3 { 3 } else { class }]
            );
            println!("  flags:   {:02x}", flags);
            println!(
                "    null device:         {}",
                if flags & OHMD_DEVICE_FLAGS_NULL_DEVICE != 0 {
                    "yes"
                } else {
                    "no"
                }
            );
            println!(
                "    rotational tracking: {}",
                if flags & OHMD_DEVICE_FLAGS_ROTATIONAL_TRACKING != 0 {
                    "yes"
                } else {
                    "no"
                }
            );
            println!(
                "    positional tracking: {}",
                if flags & OHMD_DEVICE_FLAGS_POSITIONAL_TRACKING != 0 {
                    "yes"
                } else {
                    "no"
                }
            );
            println!(
                "    left controller:     {}",
                if flags & OHMD_DEVICE_FLAGS_LEFT_CONTROLLER != 0 {
                    "yes"
                } else {
                    "no"
                }
            );
            println!(
                "    right controller:    {}",
                if flags & OHMD_DEVICE_FLAGS_RIGHT_CONTROLLER != 0 {
                    "yes"
                } else {
                    "no"
                }
            );
            println!("");
        }

        println!("opening device: {}", 0);
        let device = ohmd_list_open_device(context, 0);
        let mut out: [c_int; 1] = [0];
        ohmd_device_geti(
            device,
            ohmd_int_value::OHMD_SCREEN_HORIZONTAL_RESOLUTION,
            &mut out,
        );
        let width = out[0];
        ohmd_device_geti(
            device,
            ohmd_int_value::OHMD_SCREEN_VERTICAL_RESOLUTION,
            &mut out,
        );
        let height = out[0];
        let mut out: [c_float; 16] = [0.0; 16];
        ohmd_device_getf(
            device,
            ohmd_float_value::OHMD_SCREEN_HORIZONTAL_SIZE,
            &mut out,
        );
        let hsize = out[0];
        ohmd_device_getf(
            device,
            ohmd_float_value::OHMD_SCREEN_VERTICAL_SIZE,
            &mut out,
        );
        let vsize = out[0];
        ohmd_device_getf(
            device,
            ohmd_float_value::OHMD_LENS_HORIZONTAL_SEPARATION,
            &mut out,
        );
        let lens_separation = out[0];
        ohmd_device_getf(
            device,
            ohmd_float_value::OHMD_LENS_VERTICAL_POSITION,
            &mut out,
        );
        let lens_vcenter = out[0];
        ohmd_device_getf(device, ohmd_float_value::OHMD_LEFT_EYE_FOV, &mut out);
        let left_eye_fov = out[0];
        ohmd_device_getf(device, ohmd_float_value::OHMD_RIGHT_EYE_FOV, &mut out);
        let right_eye_fov = out[0];
        ohmd_device_getf(
            device,
            ohmd_float_value::OHMD_LEFT_EYE_ASPECT_RATIO,
            &mut out,
        );
        let left_eye_aspect = out[0];
        ohmd_device_getf(
            device,
            ohmd_float_value::OHMD_RIGHT_EYE_ASPECT_RATIO,
            &mut out,
        );
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

        for _i in 0..10000 {
            ohmd_ctx_update(context);
            ohmd_device_getf(device, ohmd_float_value::OHMD_ROTATION_QUAT, &mut out);
            println!(
                "rotation quat:           {:.6} {:.6} {:.6} {:.6}",
                out[0], out[1], out[2], out[3]
            );
            ohmd_device_getf(device, ohmd_float_value::OHMD_POSITION_VECTOR, &mut out);
            println!(
                "position vec:            {:.6} {:.6} {:.6}",
                out[0], out[1], out[2]
            );
            std::thread::sleep(Duration::from_millis(10));
        }

        ohmd_close_device(device);
        ohmd_ctx_destroy(context);
    }
}
