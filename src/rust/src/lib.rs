extern crate libc;

use std::ffi::CStr;
use std::os::raw::c_char;
use libc::size_t;
use libc::strncpy;

mod rnc {
    #[test]
    fn add_i_i() {
        assert_eq!("II", add("I", "I"));
    }

    #[test]
    fn add_i_ii() {
        assert_eq!("III", add("I", "II"));
    }

    pub fn add(num_l: &str, num_r: &str) -> String {
        let sum = String::from(num_l);
        return sum + num_r;
    }
}

use rnc::add;

#[no_mangle]
pub extern fn rnc_add(dst: *mut c_char, dstlen: size_t, num_l: *const c_char, num_r: *const c_char) {
    unsafe {
        let sum = add(CStr::from_ptr(num_l).to_str().unwrap(), CStr::from_ptr(num_r).to_str().unwrap());
        strncpy(dst, sum.as_ptr() as *const i8, dstlen);
    }
}
