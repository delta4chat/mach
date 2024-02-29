//! This module corresponds to `mach/thread_status.h`.

use crate::ffi::{c_int, c_uint};

use crate::vm_types::natural_t;

pub type thread_state_t = *mut natural_t;

#[cfg(feature = "unstable")]
pub type thread_state_flavor_t = c_uint; // from (modified) machx
#[cfg(not(feature = "unstable"))]
pub type thread_state_flavor_t = c_int;  // from (original) mach

/* List of valid flavors */
// TODO check if this type correct (compare from original C code)
pub const THREAD_STATE_FLAVOR_LIST:       thread_state_flavor_t = 0;
pub const THREAD_STATE_FLAVOR_LIST_NEW:   thread_state_flavor_t = 128;
pub const THREAD_STATE_FLAVOR_LIST_10_9:  thread_state_flavor_t = 129;
pub const THREAD_STATE_FLAVOR_LIST_10_13: thread_state_flavor_t = 130;
pub const THREAD_STATE_FLAVOR_LIST_10_15: thread_state_flavor_t = 131;

pub mod arm {
    use super::thread_state_flavor_t;

    pub const ARM_THREAD_STATE:         thread_state_flavor_t = 1;
    pub const ARM_THREAD_STATE64:       thread_state_flavor_t = 6;
    pub const ARM_UNIFIED_THREAD_STATE: thread_state_flavor_t = ARM_THREAD_STATE;

    pub const ARM_VFP_STATE:            thread_state_flavor_t = 2;
    pub const ARM_EXCEPTION_STATE:      thread_state_flavor_t = 3;
    pub const ARM_EXCEPTION_STATE64:    thread_state_flavor_t = 7;

    pub const THREAD_STATE_NONE:        thread_state_flavor_t = 5;

    pub const ARM_DEBUG_STATE:          thread_state_flavor_t = 4;
    pub const ARM_DEBUG_STATE32:        thread_state_flavor_t = 14;
    pub const ARM_DEBUG_STATE64:        thread_state_flavor_t = 15;

    pub const ARM_NEON_STATE:           thread_state_flavor_t = 16;
    pub const ARM_NEON_STATE64:         thread_state_flavor_t = 17;
    pub const ARM_CPMU_STATE64:         thread_state_flavor_t = 18;
}

pub mod x86 {
    use super::thread_state_flavor_t;

    pub const i386_THREAD_STATE:     thread_state_flavor_t = x86_THREAD_STATE32;
    pub const i386_FLOAT_STATE:      thread_state_flavor_t = x86_FLOAT_STATE32;
    pub const i386_EXCEPTION_STATE:  thread_state_flavor_t = x86_EXCEPTION_STATE32;

    /*
     * THREAD_STATE_FLAVOR_LIST 0
     *      these are the supported flavors
     */
    pub const x86_THREAD_STATE32:    thread_state_flavor_t = 1;
    pub const x86_FLOAT_STATE32:     thread_state_flavor_t = 2;
    pub const x86_EXCEPTION_STATE32: thread_state_flavor_t = 3;

    pub const x86_THREAD_STATE64:    thread_state_flavor_t = 4;
    pub const x86_FLOAT_STATE64:     thread_state_flavor_t = 5;
    pub const x86_EXCEPTION_STATE64: thread_state_flavor_t = 6;

    pub const x86_THREAD_STATE:      thread_state_flavor_t = 7;
    pub const x86_FLOAT_STATE:       thread_state_flavor_t = 8;
    pub const x86_EXCEPTION_STATE:   thread_state_flavor_t = 9;

    pub const x86_DEBUG_STATE32:     thread_state_flavor_t = 10;
    pub const x86_DEBUG_STATE64:     thread_state_flavor_t = 11;
    pub const x86_DEBUG_STATE:       thread_state_flavor_t = 12;

    pub const THREAD_STATE_NONE:     thread_state_flavor_t = 13;

    pub const x86_AVX_STATE32:    thread_state_flavor_t = 16;
    pub const x86_AVX_STATE64:    thread_state_flavor_t = x86_AVX_STATE32 + 1;
    pub const x86_AVX_STATE:      thread_state_flavor_t = x86_AVX_STATE32 + 2;

    pub const x86_AVX512_STATE32: thread_state_flavor_t = 19;
    pub const x86_AVX512_STATE64: thread_state_flavor_t = x86_AVX512_STATE32 + 1;
    pub const x86_AVX512_STATE:   thread_state_flavor_t = x86_AVX512_STATE32 + 2;

    pub const x86_PAGEIN_STATE:        thread_state_flavor_t = 22;
    pub const x86_THREAD_FULL_STATE64: thread_state_flavor_t = 23;
    pub const x86_INSTRUCTION_STATE:   thread_state_flavor_t = 24;
    pub const x86_LAST_BRANCH_STATE:   thread_state_flavor_t = 25;

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct x86_state_hdr_t {
        pub flavor: u32,
        pub count: u32,
    }
    pub type x86_state_hdr = x86_state_hdr_t;

    // TODO contiune porting from:
    //
    // https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/i386/thread_status.h#L177

    //pub const
}

#[cfg(target_arch = "aarch64")]
pub use arm::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::*;

