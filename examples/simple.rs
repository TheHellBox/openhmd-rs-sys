extern crate openhmd_rs_sys;
extern crate libc;
use openhmd_rs_sys::*;
use libc::c_float;
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

        let device = ohmd_list_open_device(context, 0);
        ohmd_ctx_update(context);
        let mut out: [c_float; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0];
        let output = ohmd_device_getf(device, ohmd_float_value::OHMD_ROTATION_QUAT, &mut out);
        println!("{:?} {}", out, output);

        ohmd_close_device(device);
        ohmd_ctx_destroy(context);
    }
}
