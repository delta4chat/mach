//! This module roughly corresponds to `mach-o/loader.h`.

use core::{ffi::c_char, mem::transmute};

pub const LC_SEGMENT: u32 = 1;
pub const LC_SEGMENT_64: u32 = 25;
pub const SEG_TEXT: &[c_char; 7usize] = unsafe { transmute(b"__TEXT\0") };
pub const SEG_LINKEDIT: &[c_char; 11usize] = unsafe { transmute(b"__LINKEDIT\0") };
pub const SEG_DATA: &[c_char; 7] = unsafe { transmute(b"__DATA\0") };
pub const SEG_DATA_CONST: &[c_char; 13] = unsafe { transmute(b"__DATA_CONST\0") };
pub const LC_SYMTAB: u32 = 2;
pub const INDIRECT_SYMBOL_LOCAL: u32 = 2147483648;
pub const INDIRECT_SYMBOL_ABS: u32 = 1073741824;
pub const SECTION_TYPE: u32 = 255;
pub const S_LAZY_SYMBOL_POINTERS: u32 = 7;
pub const S_NON_LAZY_SYMBOL_POINTERS: u32 = 6;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct load_command {
    pub cmd: u32,
    pub cmdsize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct symtab_command {
    pub cmd: u32,
    pub cmdsize: u32,
    pub symoff: u32,
    pub nsyms: u32,
    pub stroff: u32,
    pub strsize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dysymtab_command {
    pub cmd: u32,
    pub cmdsize: u32,
    pub ilocalsym: u32,
    pub nlocalsym: u32,
    pub iextdefsym: u32,
    pub nextdefsym: u32,
    pub iundefsym: u32,
    pub nundefsym: u32,
    pub tocoff: u32,
    pub ntoc: u32,
    pub modtaboff: u32,
    pub nmodtab: u32,
    pub extrefsymoff: u32,
    pub nextrefsyms: u32,
    pub indirectsymoff: u32,
    pub nindirectsyms: u32,
    pub extreloff: u32,
    pub nextrel: u32,
    pub locreloff: u32,
    pub nlocrel: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct section_64 {
    pub sectname: [::libc::c_char; 16usize],
    pub segname: [::libc::c_char; 16usize],
    pub addr: u64,
    pub size: u64,
    pub offset: u32,
    pub align: u32,
    pub reloff: u32,
    pub nreloc: u32,
    pub flags: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}
