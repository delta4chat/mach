//! This module corresponds to `mach/i386/_structs.h` and `mach/arm/_structs.h`.

use core::mem::size_of;
use core::ffi::c_int;

use crate::message::mach_msg_type_number_t;

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct arm_thread_state64_t {
    pub __x:    [u64; 29],
    pub __fp:    u64,
    pub __lr:    u64,
    pub __sp:    u64,
    pub __pc:    u64,
    pub __cpsr:  u32,
    pub __flags: u32,
}

#[cfg(target_arch = "aarch64")]
impl arm_thread_state64_t {
    pub const fn new() -> Self {
        Self {
            __x:    [0; 29],
            __fp:    0,
            __lr:    0,
            __sp:    0,
            __pc:    0,
            __cpsr:  0,
            __flags: 0,
        }
    }

    pub const fn count() -> mach_msg_type_number_t {
        let n = size_of::<Self>() / size_of::<c_int>();
        n as mach_msg_type_number_t
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct x86_thread_state64_t {
    pub __rax:    u64,
    pub __rbx:    u64,
    pub __rcx:    u64,
    pub __rdx:    u64,
    pub __rdi:    u64,
    pub __rsi:    u64,
    pub __rbp:    u64,
    pub __rsp:    u64,
    pub __r8:     u64,
    pub __r9:     u64,
    pub __r10:    u64,
    pub __r11:    u64,
    pub __r12:    u64,
    pub __r13:    u64,
    pub __r14:    u64,
    pub __r15:    u64,
    pub __rip:    u64,
    pub __rflags: u64,
    pub __cs:     u64,
    pub __fs:     u64,
    pub __gs:     u64,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl x86_thread_state64_t {
    pub const fn new() -> Self {
        Self {
            __rax:    0,
            __rbx:    0,
            __rcx:    0,
            __rdx:    0,
            __rdi:    0,
            __rsi:    0,
            __rbp:    0,
            __rsp:    0,
            __r8:     0,
            __r9:     0,
            __r10:    0,
            __r11:    0,
            __r12:    0,
            __r13:    0,
            __r14:    0,
            __r15:    0,
            __rip:    0,
            __rflags: 0,
            __cs:     0,
            __fs:     0,
            __gs:     0,
        }
    }

    pub const fn count() -> mach_msg_type_number_t {
        let n = size_of::<Self>() / size_of::<c_int>();
        n as mach_msg_type_number_t
    }
}

