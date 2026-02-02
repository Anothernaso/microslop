//! MicroSlop Foreign Function Interface for C

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::copilot::Hallucinate;
use crate::slopify::Slopify;

#[unsafe(no_mangle)]
pub extern "C" fn MICROSLOP_to_slop(src: *const c_char) -> *mut c_char {
    let c_src = unsafe {
        assert!(!src.is_null());
        CStr::from_ptr(src)
    };

    let rs_src = c_src.to_str().unwrap();

    let result = rs_src.to_string().slopify();
    let c_result = CString::new(result).unwrap();

    c_result.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn MICROSLOP_to_hallucination(src: *const c_char) -> *mut c_char {
    let c_src = unsafe {
        assert!(!src.is_null());
        CStr::from_ptr(src)
    };

    let rs_src = c_src.to_str().unwrap();

    let result = rs_src.to_string().hallucinate();
    let c_result = CString::new(result).unwrap();

    c_result.into_raw()
}
