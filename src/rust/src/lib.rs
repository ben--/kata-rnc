extern crate libc;

use libc::size_t;
use libc::strncpy;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

mod cmp;
pub use cmp::cmp;

mod denormalize;
pub use denormalize::denormalize;

mod normalize;
pub use normalize::normalize;

mod rnc;

#[no_mangle]
pub extern fn rnc_add(dst: *mut c_char, dstlen: size_t, num_l: *const c_char, num_r: *const c_char) {
    unsafe {
        let sum = rnc::add(CStr::from_ptr(num_l).to_str().unwrap(), CStr::from_ptr(num_r).to_str().unwrap());
        let csum = CString::new(sum).unwrap();
        strncpy(dst, csum.as_ptr() as *const i8, dstlen);
    }
}
