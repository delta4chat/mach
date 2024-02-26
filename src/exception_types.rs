//! This module roughly corresponds to `mach/exception_types.h`.

use core::ffi::{c_uint, c_int};

use crate::port::mach_port_t;
use crate::vm_types::integer_t;
use crate::thread_status::thread_state_flavor_t;

pub const EXC_BAD_ACCESS:           c_uint = 1;
pub const EXC_BAD_INSTRUCTION:      c_uint = 2;
pub const EXC_ARITHMETIC:           c_uint = 3;
pub const EXC_EMULATION:            c_uint = 4;
pub const EXC_SOFTWARE:             c_uint = 5;
pub const EXC_BREAKPOINT:           c_uint = 6;
pub const EXC_SYSCALL:              c_uint = 7;
pub const EXC_MACH_SYSCALL:         c_uint = 8;
pub const EXC_RPC_ALERT:            c_uint = 9;
pub const EXC_CRASH:                c_uint = 10;
pub const EXC_RESOURCE:             c_uint = 11;
pub const EXC_GUARD:                c_uint = 12;
pub const EXC_CORPSE_NOTIFY:        c_uint = 13;
pub const EXC_CORPSE_VARIANT_BIT:   c_uint = 256;
pub const EXCEPTION_DEFAULT:        c_uint = 1;
pub const EXCEPTION_STATE:          c_uint = 2;
pub const EXCEPTION_STATE_IDENTITY: c_uint = 3;
pub const MACH_EXCEPTION_CODES:     c_uint = 2_147_483_648;
pub const EXC_MASK_BAD_ACCESS:      c_uint = 2;
pub const EXC_MASK_BAD_INSTRUCTION: c_uint = 4;
pub const EXC_MASK_ARITHMETIC:      c_uint = 8;
pub const EXC_MASK_EMULATION:       c_uint = 16;
pub const EXC_MASK_SOFTWARE:        c_uint = 32;
pub const EXC_MASK_BREAKPOINT:      c_uint = 64;
pub const EXC_MASK_SYSCALL:         c_uint = 128;
pub const EXC_MASK_MACH_SYSCALL:    c_uint = 256;
pub const EXC_MASK_RPC_ALERT:       c_uint = 512;
pub const EXC_MASK_CRASH:           c_uint = 1024;
pub const EXC_MASK_RESOURCE:        c_uint = 2048;
pub const EXC_MASK_GUARD:           c_uint = 4096;
pub const EXC_MASK_CORPSE_NOTIFY:   c_uint = 8192;
pub const EXC_MASK_ALL:             c_uint = 7166;
pub const FIRST_EXCEPTION:          c_uint = 1;
pub const EXC_SOFT_SIGNAL:          c_uint = 65_539;
pub const EXC_MACF_MIN:             c_uint = 131_072;
pub const EXC_MACF_MAX:             c_uint = 196_607;

pub type exception_type_t           = c_int;
pub type exception_data_type_t      = integer_t;
pub type mach_exception_data_type_t = i64;
pub type exception_behavior_t       = c_int;
pub type exception_data_t           = *mut exception_data_type_t;
pub type mach_exception_data_t      = *mut mach_exception_data_type_t;
pub type exception_mask_t           = c_uint;
pub type exception_mask_array_t     = *mut exception_mask_t;
pub type exception_behavior_array_t = *mut exception_behavior_t;
pub type exception_flavor_array_t   = *mut thread_state_flavor_t;
pub type exception_port_array_t     = *mut mach_port_t;
pub type mach_exception_code_t      = mach_exception_data_type_t;
pub type mach_exception_subcode_t   = mach_exception_data_type_t;

