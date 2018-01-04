extern crate openhmd_rs_sys;
extern crate libc;
use openhmd_rs_sys::*;
use libc::c_float;
fn main(){
    unsafe{
        let context = ohmd_ctx_create();
        let probe = ohmd_ctx_probe(context);
        println!("probe {}", probe);
        let device = ohmd_list_open_device(context, 0);
        ohmd_ctx_update(context);
        let mut out: [c_float; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0,0.0, 0.0, 0.0, 0.0];
        let output = ohmd_device_getf(device, ohmd_float_value::OHMD_ROTATION_QUAT, &out);
        println!("{:?} {}", out, output);
    }
}
