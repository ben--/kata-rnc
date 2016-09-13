extern crate libc;

use libc::{c_int, size_t, strncpy};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

mod add;
pub use add::add;

mod borrow;
pub use borrow::borrow;

mod cmp;
pub use cmp::cmp;

mod denormalize;
pub use denormalize::denormalize;

mod normalize;
pub use normalize::normalize;

mod sub;
pub use sub::sub;

#[no_mangle]
pub extern fn rnc_add(dst: *mut c_char, dstlen: size_t, raw_l: *const c_char, raw_r: *const c_char) -> c_int {
    let num_l;
    let num_r;

    unsafe {
        num_l = CStr::from_ptr(raw_l).to_str().unwrap();
        num_r = CStr::from_ptr(raw_r).to_str().unwrap();
    }

    match add(num_l, num_r) {
        Ok(sum) => {
            if sum.len() > (dstlen - 1) {
                1
            } else {
                let csum = CString::new(sum).unwrap();
                unsafe {
                    strncpy(dst, csum.as_ptr() as *const i8, dstlen);
                }
                0
            }
        },
        Err(_) => {
            1
        }
    }
}

#[no_mangle]
pub extern fn rnc_sub(dst: *mut c_char, dstlen: size_t, raw_l: *const c_char, raw_r: *const c_char) -> c_int {
    let num_l;
    let num_r;

    unsafe {
        num_l = CStr::from_ptr(raw_l).to_str().unwrap();
        num_r = CStr::from_ptr(raw_r).to_str().unwrap();
    }

    match sub(num_l, num_r) {
        Ok(difference) => {
            //if sum.len() > (dstlen - 1) {
            //1
            //} else {
            let cdiff = CString::new(difference).unwrap();
            unsafe {
                strncpy(dst, cdiff.as_ptr() as *const i8, dstlen);
            }
            0
        },
        Err(_) => {
            1
        }
    }
}
