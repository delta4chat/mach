//! This module roughly corresponds to `mach/clock_types.h`.

use core::ffi::{c_int, c_uint, c_ulonglong};

pub type alarm_type_t   = c_int;
pub type sleep_type_t   = c_int;
pub type clock_id_t     = c_int;
pub type clock_flavor_t = c_int;
pub type clock_attr_t   = *mut c_int;
pub type clock_res_t    = c_int;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct mach_timespec_t {
    pub tv_sec:  c_uint,
    pub tv_nsec: clock_res_t,
}
pub type mach_timespec = mach_timespec_t;

impl mach_timespec_t {
    pub const fn new() -> Self {
        Self {
            tv_sec:  0,
            tv_nsec: 0,
        }
    }

    pub const fn is_valid(&self) -> bool {
        if self.tv_nsec < 0 {
            return false;
        }

        if (self.tv_nsec as c_ulonglong) >=NSEC_PER_SEC{
            return false;
        }

        true
    }

    pub const fn cmp(&self, other: &Self) ->c_ulonglong{
        if self.tv_sec > other.tv_sec {
            return NSEC_PER_SEC;
        }

        if self.tv_sec < other.tv_sec {
            return !NSEC_PER_SEC;
        }

        (self.tv_nsec - other.tv_nsec) as c_ulonglong
    }

    pub const fn add(&self, other: &Self) -> Self {
        let mut result = Self::new();

        result.tv_sec  = self.tv_sec  + other.tv_sec;
        result.tv_nsec = self.tv_nsec + other.tv_nsec;

        let t1ns = self.tv_nsec as c_ulonglong;
        if t1ns >= NSEC_PER_SEC {
            result.tv_nsec =
                (t1ns - NSEC_PER_SEC) as clock_res_t;

            result.tv_sec += 1;
        }

        result
    }

    pub const fn sub(&self, other: &Self) -> Self {
        let mut result = Self::new();

        result.tv_sec  = self.tv_sec  - other.tv_sec;
        result.tv_nsec = self.tv_nsec - other.tv_nsec;

        if result.tv_nsec < 0 {
            result.tv_nsec +=
                NSEC_PER_SEC as clock_res_t;

            result.tv_sec  -= 1;
        }

        result
    }
}

use core::ops::{Add, Sub};

macro_rules! _timespec_ops_impl {
    ($op:tt, $trait_func:tt, $orig_func:tt) => {
        impl $op <mach_timespec> for mach_timespec {
            type Output = Self;
            fn $trait_func (self, other: mach_timespec)
                -> mach_timespec
            {
                mach_timespec::$orig_func(&self, &other)
            }
        }
        //* FIXME resolve lifetime incompatible [E0308]:
        impl $op <&mach_timespec> for &mach_timespec {
            type Output = mach_timespec;
            fn $trait_func (self, other: &mach_timespec)
                -> mach_timespec
            {
                mach_timespec::$orig_func (self, other)
            }
        }
        
        impl $op <&mach_timespec> for mach_timespec {
            type Output = Self;
            fn $trait_func (self, other: &mach_timespec)
                -> mach_timespec
            {
                mach_timespec::$orig_func (&self, other)
            }
        }
        impl $op <mach_timespec> for &mach_timespec {
            type Output = mach_timespec;
            fn $trait_func (self, other: mach_timespec)
                -> mach_timespec
            {
                mach_timespec::$orig_func (self, &other)
            }
        }
    }
}

_timespec_ops_impl!(Add, add, add);
_timespec_ops_impl!(Sub, sub, sub);

pub const SYSTEM_CLOCK:   c_uint = 0;
pub const CALENDAR_CLOCK: c_uint = 1;
pub const REALTIME_CLOCK: c_uint = 0;

pub const CLOCK_GET_TIME_RES: c_uint = 1;
pub const CLOCK_ALARM_CURRES: c_uint = 3;
pub const CLOCK_ALARM_MINRES: c_uint = 4;
pub const CLOCK_ALARM_MAXRES: c_uint = 5;


pub const USEC_PER_SEC:  c_ulonglong = 1_000_000;
pub const NSEC_PER_SEC:  c_ulonglong = 1_000_000_000;

pub const NSEC_PER_USEC: c_ulonglong = 1_000;
pub const NSEC_PER_MSEC: c_ulonglong = 1_000_000;

#[allow(non_snake_case)]
pub const fn BAD_MACH_TIMESPEC(t: mach_timespec) ->bool{
    ! t.is_valid()
}

#[allow(non_snake_case)]
pub const fn CMP_MACH_TIMESPEC(
    t1: &mach_timespec,
    t2: &mach_timespec
) -> c_ulonglong {
    t1.cmp(t2)
}

pub const fn add_mach_timespec(
    t1: &mach_timespec,
    t2: &mach_timespec
) -> mach_timespec {
    mach_timespec::add(t1, t2)
}
pub const fn sub_mach_timespec(
    t1: &mach_timespec,
    t2: &mach_timespec
) -> mach_timespec {
    mach_timespec::sub(t1, t2)
}

#[allow(non_snake_case)]
#[deprecated]
pub fn ADD_MACH_TIMESPEC(
    t1: &mut mach_timespec,
    t2: &mach_timespec
) {
    *t1 = mach_timespec::add(t1, t2);
}
#[allow(non_snake_case)]
#[deprecated]
pub fn SUB_MACH_TIMESPEC(
    t1: &mut mach_timespec,
    t2: &mach_timespec
) {
    *t1 = mach_timespec::sub(t1, t2);
}

pub const ALRMTYPE:      c_uint = 0xff;
pub const TIME_ABSOLUTE: c_uint = 0x00;
pub const TIME_RELATIVE: c_uint = 0x01;

#[allow(non_snake_case)]
pub const fn BAD_ALRMTYPE(t: c_uint) -> bool {
    ( t & (!TIME_RELATIVE) ) != 0
}

