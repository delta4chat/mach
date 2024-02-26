//! This module roughly corresponds to `mach-o/loader.h`.

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

