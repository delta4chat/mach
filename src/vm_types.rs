//! This module roughly corresponds to `mach/i386/vm_types.h` (on amd64)
//! and `mach/arm/vm_types.h` (on aarch64).

use core::ffi::{c_uint, c_int};
type uintptr_t = usize; // in all platforms, `uintptr_t` is equiv to `usize`

use crate::port::mach_port_t;

pub type natural_t = c_uint;
pub type integer_t = c_int;

pub type vm_map_t            = mach_port_t;

pub type user_addr_t         = u64;
pub type mach_vm_address_t   = u64;
pub type vm_map_address_t    = u64;
pub type vm_address_t        = vm_offset_t;

pub type mach_vm_offset_t    = u64;
pub type vm_map_offset_t     = u64;
pub type vm_offset_t         = uintptr_t;

pub type mach_vm_size_t      = u64;
pub type vm_map_size_t       = u64;
pub type vm_size_t           = uintptr_t;

pub type mach_port_context_t = mach_vm_address_t;

