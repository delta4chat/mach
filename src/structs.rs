//! This module corresponds to `mach/i386/_structs.h` and `mach/arm/_structs.h`.

use core::mem::size_of;
use crate::ffi::c_int;

use crate::message::mach_msg_type_number_t;

// re-export if compile for aarch64/arm64 CPUs
#[cfg(target_arch = "aarch64")]
pub use arm::*;

// re-export if compile for x86 or x86-64 CPUs
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::*;

/* 
 * but these `mod` both always exists whatever which target arch.
 *
 * in some cases, this maybe helps for cross-platforms.
 */

pub mod arm {
    use super::mach_msg_type_number_t;

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
    pub struct arm_state_hdr_t {
        pub flavor: u32,
        pub count:  u32,
    }
    pub type   arm_state_hdr = arm_state_hdr_t;

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Default, Hash)]
    pub struct arm_unified_thread_state_t {
        pub ash: arm_state_hdr_t,
        pub uts: arm_unified_thread_state__bindgen_ty_1,
    }
    pub type   arm_unified_thread_state = arm_unified_thread_state_t;

    impl arm_unified_thread_state_t {
        pub const fn from_32(flavor: u32, ts_32: arm_thread_state32_t) -> Self {
            let count = arm_thread_state32_t::count() as u32;

            let ash = arm_state_hdr_t { flavor, count };
            let uts = arm_thread_state32or64_t { ts_32 };

            Self { ash, uts }
        }
        pub const fn from_64(flavor: u32, ts_64: arm_thread_state64_t) -> Self {
            let count = arm_thread_state64_t::count() as u32;

            let ash = arm_state_hdr_t { flavor, count };
            let uts = arm_thread_state32or64_t { ts_64 };

            Self { ash, uts }
        }
    }

    #[repr(C)]
    pub union arm_thread_state32or64_t {
        pub ts_32: arm_thread_state32_t,
        pub ts_64: arm_thread_state64_t,
    }
    pub type arm_thread_state32or64                 = arm_thread_state32or64_t;
    pub type arm_unified_thread_state__bindgen_ty_1 = arm_thread_state32or64_t;

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
    pub struct arm_thread_state32_t {
        pub __r:   [u32; 13usize],
        pub __sp:   u32,
        pub __lr:   u32,
        pub __pc:   u32,
        pub __cpsr: u32,
    }
    pub type arm_thread_state32        = arm_thread_state32_t;
    pub type __darwin_arm_thread_state = arm_thread_state32_t;

    impl arm_thread_state32_t {
        pub const fn new() -> Self {
            Self {
                __r:   [0; 13],
                __sp:   0,
                __lr:   0,
                __pc:   0,
                __cpsr: 0,
            }
        }

        pub const fn count() -> mach_msg_type_number_t {
            let n = size_of::<Self>() / size_of::<u32>();
            n as mach_msg_type_number_t
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
    pub struct arm_thread_state64_t {
        pub __x:   [u64; 29usize],
        pub __fp:   u64,
        pub __lr:   u64,
        pub __sp:   u64,
        pub __pc:   u64,
        pub __cpsr: u32,
        pub __pad:  u32,
    }
    pub type arm_thread_state64          = arm_thread_state64_t;
    pub type __darwin_arm_thread_state64 = arm_thread_state64_t;

    impl arm_thread_state64_t {
        pub const fn new() -> Self {
            Self {
                __x:   [0; 29],
                __fp:   0,
                __lr:   0,
                __sp:   0,
                __pc:   0,
                __cpsr: 0,
                __pad:  0,
            }
        }

        pub const fn count() -> mach_msg_type_number_t {
            let n = size_of::<Self>() / size_of::<u64>();
            n as mach_msg_type_number_t
        }
    }
}

pub mod x86 {
    use super::mach_msg_type_number_t;

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
            let n = size_of::<Self>() / size_of::<u64>();
            n as mach_msg_type_number_t
        }
    }
}

