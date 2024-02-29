//! This module roughly corresponds to `mach-o/loader.h`.

use core::ffi::{c_char, CStr};

use crate::vm_types::integer_t;

// in all apple/darwin platforms,
// type `cpu_type_t` and `cpu_subtype_t`,
// both equiv to `integer_t`.
type cpu_type_t    = integer_t;
type cpu_subtype_t = integer_t;

#[repr(C)]
#[allow(dead_code, non_snake_case)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct mach_header {
    pub magic:      u32,
    pub cputype:    cpu_type_t,
    pub cpusubtype: cpu_subtype_t,
    pub filetype:   u32,
    pub ncmds:      u32,
    pub sizeofcmds: u32,
    pub flags:      u32,
}

pub const LC_SEGMENT:                 u32 = 1;
pub const LC_SEGMENT_64:              u32 = 25;
pub const LC_SYMTAB:                  u32 = 2;
pub const INDIRECT_SYMBOL_LOCAL:      u32 = 2147483648;
pub const INDIRECT_SYMBOL_ABS:        u32 = 1073741824;
pub const SECTION_TYPE:               u32 = 255;
pub const S_LAZY_SYMBOL_POINTERS:     u32 = 7;
pub const S_NON_LAZY_SYMBOL_POINTERS: u32 = 6;


pub mod bytes {
    pub const SEG_TEXT:       &[u8; 7]  = b"__TEXT\0";
    pub const SEG_DATA:       &[u8; 7]  = b"__DATA\0";
    pub const SEG_LINKEDIT:   &[u8; 11] = b"__LINKEDIT\0";
    pub const SEG_DATA_CONST: &[u8; 13] = b"__DATA_CONST\0";
}

pub mod cstr {
    // this must not to "pub"
    const fn bytes_to_cstr(bytes: &[u8]) -> CStr {
        /// SAFETY: provided slice must with NUL-terminated or no any interior-NUL-bytes.
        unsafe { CStr::from_bytes_with_nul_unchecked(bytes) }
    }

    use super::bytes;
    pub const SEG_TEXT:       CStr = bytes_to_cstr(bytes::SEG_TEXT);
    pub const SEG_DATA:       CStr = bytes_to_cstr(bytes::SEG_DATA);
    pub const SEG_LINKEDIT:   CStr = bytes_to_cstr(bytes::SEG_LINKEDIT);
    pub const SEG_DATA_CONST: CStr = bytes_to_cstr(bytes::SED_DATA_CONST);
}

#[cfg(feature = "unstable")]
pub use cstr::*;

#[cfg(not(feature = "unstable"))]
pub use bytes::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct load_command {
    pub cmd:     u32,
    pub cmdsize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct symtab_command {
    pub cmd:     u32,
    pub cmdsize: u32,
    pub symoff:  u32,
    pub nsyms:   u32,
    pub stroff:  u32,
    pub strsize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct dysymtab_command {
    pub cmd:            u32,
    pub cmdsize:        u32,
    pub ilocalsym:      u32,
    pub nlocalsym:      u32,
    pub iextdefsym:     u32,
    pub nextdefsym:     u32,
    pub iundefsym:      u32,
    pub nundefsym:      u32,
    pub tocoff:         u32,
    pub ntoc:           u32,
    pub modtaboff:      u32,
    pub nmodtab:        u32,
    pub extrefsymoff:   u32,
    pub nextrefsyms:    u32,
    pub indirectsymoff: u32,
    pub nindirectsyms:  u32,
    pub extreloff:      u32,
    pub nextrel:        u32,
    pub locreloff:      u32,
    pub nlocrel:        u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct section_64 {
    pub sectname:  [c_char; 16],
    pub segname:   [c_char; 16],
    pub addr:      u64,
    pub size:      u64,
    pub offset:    u32,
    pub align:     u32,
    pub reloff:    u32,
    pub nreloc:    u32,
    pub flags:     u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}

