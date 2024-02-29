//! This module roughly corresponds to `mach/mach_error.h`.

use crate::ffi::c_char;

use crate::kern_return::kern_return_t;

pub type mach_error_t = kern_return_t;

extern "C" {
    pub fn mach_error_string(error_value: mach_error_t) -> *mut c_char;
}

