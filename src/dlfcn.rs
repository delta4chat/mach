//! This module roughly corresponds to `dlfcn.h`.

use crate::ffi::*;

pub type Dl_info = dl_info;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dl_info {
    pub dli_fname: *const c_char,
    pub dli_fbase: *mut c_void,
    pub dli_sname: *const c_char,
    pub dli_saddr: *mut c_void,
}

extern "C" {
    pub fn dladdr(arg1: *const c_void, arg2: *mut Dl_info) -> c_int;
    pub fn dlclose(__handle: *mut c_void) -> c_int;
    pub fn dlerror() -> *mut c_char;
    pub fn dlopen(__path: *const c_char, __mode: c_int) -> *mut c_void;
    pub fn dlsym(__handle: *mut c_void, __symbol: *const c_char) -> *mut c_void;
    pub fn dlopen_preflight(__path: *const c_char) -> bool;
}
