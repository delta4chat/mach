//! This module roughly corresponds to `mach-o/dyld_images.h`.

#![allow(non_snake_case)]

use crate::ffi::{
    c_uint, c_ulong,
    c_int,
    c_uchar, c_char,
    c_void,
};
use crate::ffi::uintptr_t;

use crate::mach_types::uuid_t;
use crate::port::mach_port_t;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_header {
    pub magic: c_uint,
    pub cputype: c_int,
    pub cpusubtype: c_int,
    pub filetype: c_uint,
    pub ncmds: c_uint,
    pub sizeofcmds: c_uint,
    pub flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_header_64 {
    pub magic: c_uint,
    pub cputype: c_int,
    pub cpusubtype: c_int,
    pub filetype: c_uint,
    pub ncmds: c_uint,
    pub sizeofcmds: c_uint,
    pub flags: c_uint,
    pub reserved: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct segment_command_64 {
    pub cmd: c_uint,
    pub cmdsize: c_uint,
    pub segname: [c_char; 16],
    pub vmaddr: c_ulong,
    pub vmsize: c_ulong,
    pub fileoff: c_ulong,
    pub filesize: c_ulong,
    pub maxprot: c_int,
    pub initprot: c_int,
    pub nsects: c_uint,
    pub flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_uuid_info {
    pub imageLoadAddress: *const mach_header,
    pub imageUUID: uuid_t,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_aot_image_info {
    pub x86LoadAddress: *const mach_header,
    pub aotLoadAddress: *const mach_header,
    pub aotImageSize: c_ulong,
    pub aotImageKey: [c_uchar; 32],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_image_info {
    pub imageLoadAddress: *mut mach_header,
    pub imageFilePath: *const c_char,
    pub imageFileModDate: uintptr_t,
}

pub type dyld_image_mode = c_uint;

pub type dyld_image_notifier = ::core::option::Option<
    unsafe extern "C" fn(mode: dyld_image_mode, infoCount: c_uint, info: *const dyld_image_info),
>;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct dyld_all_image_infos {
    pub version: c_uint,
    pub infoArrayCount: c_uint,
    pub infoArray: *const dyld_image_info,
    pub notification: dyld_image_notifier,
    pub processDetachedFromSharedRegion: bool,
    pub libSystemInitialized: bool,
    pub dyldImageLoadAddress: *const mach_header,
    pub jitInfo: *mut c_void,
    pub dyldVersion: *const c_char,
    pub errorMessage: *const c_char,
    pub terminationFlags: uintptr_t,
    pub coreSymbolicationShmPage: *mut c_void,
    pub systemOrderFlag: uintptr_t,
    pub uuidArrayCount: uintptr_t,
    pub uuidArray: *const dyld_uuid_info,
    pub dyldAllImageInfosAddress: *mut dyld_all_image_infos,
    pub initialImageCount: uintptr_t,
    pub errorKind: uintptr_t,
    pub errorClientOfDylibPath: *const c_char,
    pub errorTargetDylibPath: *const c_char,
    pub errorSymbol: *const c_char,
    pub sharedCacheSlide: uintptr_t,
    pub sharedCacheUUID: [c_uchar; 16],
    pub sharedCacheBaseAddress: uintptr_t,
    pub infoArrayChangeTimestamp: c_ulong,
    pub dyldPath: *const c_char,
    pub notifyPorts: [mach_port_t; 8],
    pub reserved: [uintptr_t; 7],
    pub sharedCacheFSID: c_ulong,
    pub sharedCacheFSObjID: c_ulong,
    pub compact_dyld_image_info_addr: uintptr_t,
    pub compact_dyld_image_info_size: uintptr_t,
    pub platform: c_uint,
    pub aotInfoCount: c_uint,
    pub aotInfoArray: *const dyld_aot_image_info,
    pub aotInfoArrayChangeTimestamp: c_ulong,
    pub aotSharedCacheBaseAddress: uintptr_t,
    pub aotSharedCacheUUID: [c_uchar; 16],
}
