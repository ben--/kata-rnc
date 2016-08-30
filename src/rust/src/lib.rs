extern crate libc;

use libc::{c_int, size_t, strncpy};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

mod add;
pub use add::add;

mod cmp;
pub use cmp::cmp;

mod denormalize;
pub use denormalize::denormalize;

mod normalize;
pub use normalize::normalize;

mod sub;
pub use sub::sub;

#[no_mangle]
pub extern fn rnc_add(dst: *mut c_char, dstlen: size_t, num_l: *const c_char, num_r: *const c_char) -> c_int {
    unsafe {
        let sum = add(CStr::from_ptr(num_l).to_str().unwrap(), CStr::from_ptr(num_r).to_str().unwrap());
        if sum.len() > (dstlen - 1) {
            1
        } else {
            let csum = CString::new(sum).unwrap();
            strncpy(dst, csum.as_ptr() as *const i8, dstlen);
            0
        }
    }
}

#[no_mangle]
pub extern fn rnc_sub(dst: *mut c_char, dstlen: size_t, num_l: *const c_char, num_r: *const c_char) -> c_int {
    unsafe {
        let diff = sub(CStr::from_ptr(num_l).to_str().unwrap(), CStr::from_ptr(num_r).to_str().unwrap());
        //if sum.len() > (dstlen - 1) {
            //1
        //} else {
            let cdiff = CString::new(diff).unwrap();
            strncpy(dst, cdiff.as_ptr() as *const i8, dstlen);
            0
        //}
    }
}
