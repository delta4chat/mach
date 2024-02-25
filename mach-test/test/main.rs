#![allow(bad_style)]

extern crate libc;
extern crate mach_sys;

use mach_sys::boolean::*;
use mach_sys::bootstrap::*;
use mach_sys::clock::*;
use mach_sys::clock_priv::*;
use mach_sys::clock_reply::*;
use mach_sys::clock_types::*;
use mach_sys::dyld::*;
use mach_sys::dyld_kernel::*;
use mach_sys::exc::*;
use mach_sys::exception_types::*;
use mach_sys::kern_return::*;
use mach_sys::loader::*;
use mach_sys::mach_init::*;
use mach_sys::mach_port::*;
use mach_sys::mach_time::*;
use mach_sys::mach_types::*;
use mach_sys::memory_object_types::*;
use mach_sys::message::*;
use mach_sys::ndr::*;
use mach_sys::port::*;
use mach_sys::semaphore::*;
use mach_sys::structs::*;
use mach_sys::sync_policy::*;
use mach_sys::task::*;
use mach_sys::task_info::*;
use mach_sys::thread_act::*;
use mach_sys::thread_policy::*;
use mach_sys::thread_status::*;
use mach_sys::time_value::*;
use mach_sys::traps::*;
use mach_sys::vm::*;
use mach_sys::vm_attributes::*;
use mach_sys::vm_behavior::*;
use mach_sys::vm_inherit::*;
// FIXME: vm_page_size is not used => not tested?
#[allow(unused_imports)]
use mach_sys::vm_page_size::*;
use mach_sys::vm_prot::*;
use mach_sys::vm_purgable::*;
use mach_sys::vm_region::*;
use mach_sys::vm_statistics::*;
use mach_sys::vm_sync::*;
use mach_sys::vm_types::*;

// These types are not re-exported by mach_sys::types but they are required.
use libc::{c_int, c_uchar, c_uint, c_ulonglong, clock_t};

// Imported by mach, but kept private:
extern "C" {
    static mach_task_self_: mach_port_t;
}

include!(concat!(env!("OUT_DIR"), "/all.rs"));
