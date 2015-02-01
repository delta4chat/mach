//! This module corresponds to `mach/port.h`

use vm_types::{natural_t};

#[repr(C)]
pub type mach_port_name_t = natural_t;

#[repr(C)]
struct ipc_port;

#[repr(C)]
pub type ipc_port_t = *mut ipc_port;

#[repr(C)]
pub type mach_port_t = ipc_port_t;