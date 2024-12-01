pub mod api;
pub mod data_processing;
pub mod metrics;
pub mod models;
pub mod utils;

//#Todo add other modules as needed

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FfiError {
    #[error("Null input provided")]
    NullInput,
    #[error("Failed to convert C string to Rust string")]
    InvalidUtf8,
    #[error("Failed to create C string")]
    CStrError,
}

#[no_mangle]
pub extern "C" fn process_input(input: *const c_char) -> *mut c_char {
    match unsafe { process_input_safe(input) } {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

fn process_input_safe(input: *const c_char) -> Result<CString, FfiError> {
    if input.is_null() {
        return Err(FfiError::NullInput);
    }

    let c_str = unsafe { CStr::from_ptr(input) };
    let r_str = c_str.to_str().map_err(|_| FfiError::InvalidUtf8)?;
    let processed = format!("Processed: {}", r_str);
    CString::new(processed).map_err(|_| FfiError::CStrError)
}

/// Frees a CString allocated in Rust to prevent memory leaks.
#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        CString::from_raw(s);
    }
}
