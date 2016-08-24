extern crate libc;

use libc::size_t;
use libc::strncpy;
use std::ffi::CStr;
use std::os::raw::c_char;

mod rnc;

#[no_mangle]
pub extern fn rnc_add(dst: *mut c_char, dstlen: size_t, num_l: *const c_char, num_r: *const c_char) {
    unsafe {
        let sum = rnc::add(CStr::from_ptr(num_l).to_str().unwrap(), CStr::from_ptr(num_r).to_str().unwrap());
        strncpy(dst, sum.as_ptr() as *const i8, dstlen);
    }
}
