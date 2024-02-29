//! This module corresponds to `mach/i386/boolean.h`.

#![allow(unused_imports)]

use crate::ffi::{c_uint, c_int};

#[cfg(target_arch = "x86_64")]
pub type boolean_t = c_uint;

#[cfg(not(target_arch = "x86_64"))]
pub type boolean_t = c_int;

