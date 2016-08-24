extern crate libc;

use std::ffi::CStr;
use std::os::raw::c_char;
use libc::size_t;
use libc::strncpy;

#[test]
fn add_i_i() {
    let actual_value = _add("I", "I");

    assert_eq!("II", actual_value);
}

#[test]
fn add_i_ii() {
    let actual_value = _add("I", "II");

    assert_eq!("III", actual_value);
}

#[no_mangle]
pub extern fn rnc_add(dst: *mut c_char, dstlen: size_t, num_l: *const c_char, num_r: *const c_char) {
    unsafe {
        let sum = _add(CStr::from_ptr(num_l).to_str().unwrap(), CStr::from_ptr(num_r).to_str().unwrap());
        strncpy(dst, sum.as_ptr() as *const i8, dstlen);
    }
}

fn _add(num_l: &str, num_r: &str) -> String {
    let sum = String::from(num_l);
    return sum + num_r;
}
