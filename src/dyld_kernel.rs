//! This module roughly corresponds to `mach/dyld_kernel.h`.

#![allow(non_snake_case)]

use crate::boolean::boolean_t;
use crate::mach_types::{fsid_t, fsobj_id_t, uuid_t};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_kernel_image_info_t {
    pub uuid:      uuid_t,
    pub fsobjid:   fsobj_id_t,
    pub fsid:      fsid_t,
    pub load_addr: u64,
}
pub type dyld_kernel_image_info         = dyld_kernel_image_info_t;
pub type dyld_kernel_image_info_array_t = *mut dyld_kernel_image_info_t;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_kernel_process_info_t {
    pub cache_image_info:  dyld_kernel_image_info_t,
    pub timestamp:         u64,
    pub imageCount:        u32,
    pub initialImageCount: u32,
    pub dyldState:         u8,
    pub no_cache:          boolean_t,
    pub private_cache:     boolean_t,
}
pub type dyld_kernel_process_info = dyld_kernel_process_info_t;

