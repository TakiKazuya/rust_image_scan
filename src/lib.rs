extern crate core;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern fn run(src_path: *const c_char, dst_path: *const c_char) -> *const c_char {
    let src_path = unsafe { CStr::from_ptr(src_path).to_str().unwrap() };
    let dst_path = unsafe { CStr::from_ptr(dst_path).to_str().unwrap() };

    println!("src_path: {:?}", &src_path);
    println!("dst_path: {:?}", &dst_path);

    CString::new(dst_path).unwrap().into_raw()
}