//! This module roughly corresponds to `mach-o/loader.h`.

use crate::ffi::{
    c_char,
    cpu_type_t,
    cpu_subtype_t,
};
use core::mem::transmute;

// a fixed size, array of signed char (8-bits)
macro_rules! _c_char_arr {
    ($name:ident, $bytes:expr) => {
        pub const $name: &[c_char; $bytes.len()] = unsafe { transmute($bytes) };
    }
}

_c_char_arr!(SEG_TEXT, b"__TEXT\0");
_c_char_arr!(SEG_LINKEDIT, b"__LINKEDIT\0");
_c_char_arr!(SEG_DATA, b"__DATA\0");
_c_char_arr!(SEG_DATA_CONST, b"__DATA_CONST\0");

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

