//! This module roughly corresponds to `mach/ndr.h`.

use core::ffi::c_uchar;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct NDR_record_t {
    pub mig_vers:     c_uchar,
    pub if_vers:      c_uchar,
    pub reserved1:    c_uchar,
    pub mig_encoding: c_uchar,
    pub int_rep:      c_uchar,
    pub char_rep:     c_uchar,
    pub float_rep:    c_uchar,
    pub reserved32:   c_uchar,
}

extern "C" {
    pub static NDR_record: NDR_record_t;
}
