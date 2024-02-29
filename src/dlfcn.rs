//! This module roughly corresponds to `dlfcn.h`.

pub type Dl_info = dl_info;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dl_info {
    pub dli_fname: *const ::libc::c_char,
    pub dli_fbase: *mut ::libc::c_void,
    pub dli_sname: *const ::libc::c_char,
    pub dli_saddr: *mut ::libc::c_void,
}

extern "C" {
    pub fn dladdr(arg1: *const ::libc::c_void, arg2: *mut Dl_info) -> ::libc::c_int;
    pub fn dlclose(__handle: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn dlerror() -> *mut ::libc::c_char;
    pub fn dlopen(__path: *const ::libc::c_char, __mode: ::libc::c_int) -> *mut ::libc::c_void;
    pub fn dlsym(__handle: *mut ::libc::c_void, __symbol: *const ::libc::c_char) -> *mut ::libc::c_void;
    pub fn dlopen_preflight(__path: *const ::libc::c_char) -> bool;
}
