extern crate libc;

use std::os::raw::c_char;
use libc::size_t;
use libc::strncpy;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}

#[no_mangle]
pub extern fn rnc_add(dst: *mut c_char, dstlen: size_t, num_l: *const c_char, num_r: *const c_char) {
    let x = "II";

    unsafe {
        strncpy(dst, x.as_ptr() as *const i8, dstlen);
    }
}
