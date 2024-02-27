//! This module roughly corresponds to `mach/clock_types.h`.

use core::ffi::{c_int, c_uint, c_ulonglong};
use core::ops::{Add, Sub, AddAssign, SubAssign};
use core::cmp::Ordering;
use core::time::Duration;

pub type alarm_type_t   = c_int;
pub type sleep_type_t   = c_int;
pub type clock_id_t     = c_int;
pub type clock_flavor_t = c_int;
pub type clock_attr_t   = *mut c_int;
pub type clock_res_t    = c_int;

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

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq)]
pub struct mach_timespec_t {
    pub tv_sec:  c_uint,
    pub tv_nsec: clock_res_t,
}
pub type mach_timespec = mach_timespec_t;

impl Ord for mach_timespec {
    fn cmp(&self, other: &mach_timespec) -> Ordering {
        mach_timespec::cmp(self, other)
    }
}

impl From<mach_timespec> for Duration {
    fn from(val: mach_timespec) -> Duration {
        val.to_duration()
    }
}
impl From<Duration> for mach_timespec {
    fn from(val: Duration) -> mach_timespec {
        mach_timespec::from_duration(val)
    }
}

impl mach_timespec {
    pub const fn new() -> Self {
        Self {
            tv_sec:  0,
            tv_nsec: 0,
        }
    }

    pub const fn from_secs(sec: c_uint) -> Self {
        Self {
            tv_sec:  sec,
            tv_nsec: 0
        }
    }
    pub const fn from_nanos(nsec: clock_res_t) -> Self {
        if nsec < 0 {
            panic!("mach_timespec cannot convert from nanoseconds: `.tv_nsec` is a negative integer!");
        }
        Self::from_duration( Duration::from_nanos(nsec as u64) )
    }

    /* start non-const methods */
    pub fn from_secs_f64(fsec: f64) -> Self {
        Self::from_duration( Duration::from_secs_f64(fsec) )
    }
    pub fn from_secs_f32(fsec: f32) -> Self {
        Self::from_duration( Duration::from_secs_f32(fsec) )
    }
    /* end non-const methods */

    pub const fn to_duration(&self) -> Duration {
        if ! self.is_valid() {
            panic!("mach_timespec is invalid!");
        }

        let sec  = self.tv_sec  as u64;
        let nsec = self.tv_nsec as u32;

        Duration::new(sec, nsec)
    }
    pub const fn from_duration(dur: Duration) -> Self {
        let tv_sec = {
            let sec = dur.as_secs();

            // handle 2038 year issue (aka Y2038 problem)
            if sec > (c_uint::MAX as u64) {
                panic!("cannot convert from Duration to mach_timespec: .tv_sec overflow c_uint!");
            }

            sec as c_uint
        };

        // this is fine.
        // because 1,000,000,000 never overflow c_int (even it's 32-bit).
        // and the return value is never be negative number: https://doc.rust-lang.org/1.76.0/src/core/time.rs.html#36
        let tv_nsec = dur.subsec_nanos() as clock_res_t;

        let this = Self {
            tv_sec,
            tv_nsec,
        };
        assert!( this.is_valid() ); // this should never fails (otherwise found a bug)
        this
    }

    pub const fn is_valid(&self) -> bool {
        if self.tv_nsec < 0 {
            return false;
        }

        if (self.tv_nsec as c_ulonglong) >= NSEC_PER_SEC {
            return false;
        }

        true
    }

    pub const fn cmp(&self, other: &Self) -> Ordering {
        if self.tv_sec > other.tv_sec {
            return Ordering::Greater;
        }
        if self.tv_sec < other.tv_sec {
            return Ordering::Less;
        }

        // self.tv_sec == other.tv_sec

        if self.tv_nsec > other.tv_nsec {
            return Ordering::Greater;
        }
        if self.tv_nsec < other.tv_nsec {
            return Ordering::Less;
        }

        // both of seconds and nano-seconds is equal.

        Ordering::Equal
    }

    pub const fn nsec_diff(&self, other: &Self)
        -> c_ulonglong
    {
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

macro_rules! _timespec_ops_impl {
    ($op:tt, $func:tt) => {
        _timespec_ops_impl!($op, $func, $func);
    };

    ($op:tt, $trait_func:tt, $orig_func:tt) => {
        impl $op<mach_timespec> for mach_timespec {
            type Output = mach_timespec;
            fn $trait_func(self, other: mach_timespec)
                -> mach_timespec
            {
                mach_timespec::$orig_func(&self, &other)
            }
        }

        // we can not use Self (due to it contains borrow symbol)
        impl $op<&mach_timespec> for &mach_timespec {
            type Output = mach_timespec;
            fn $trait_func(self, other: &mach_timespec)
                -> mach_timespec
            {
                mach_timespec::$orig_func(self, other)
            }
        }
        
        impl $op<&mach_timespec> for mach_timespec {
            type Output = mach_timespec;
            fn $trait_func(self, other: &mach_timespec)
                -> mach_timespec
            {
                mach_timespec::$orig_func(&self, other)
            }
        }
        impl $op<mach_timespec> for &mach_timespec {
            type Output = mach_timespec;
            fn $trait_func(self, other: mach_timespec)
                -> mach_timespec
            {
                mach_timespec::$orig_func(self, &other)
            }
        }
    };
}

_timespec_ops_impl!(Add, add);
_timespec_ops_impl!(Sub, sub);

macro_rules! _timespec_ops_assign_impl {
    ($op:tt, $func:tt, $orig_func:tt) => {
        impl $op for mach_timespec {
            fn $func(&mut self, other: mach_timespec) {
                let ret = mach_timespec::$orig_func(&self, &other);
                *self = ret;
            }
        }
    }
}

_timespec_ops_assign_impl!(AddAssign, add_assign, add);
_timespec_ops_assign_impl!(SubAssign, sub_assign, sub);

/* legacy functions */

#[allow(non_snake_case)]
pub const fn BAD_MACH_TIMESPEC(t: mach_timespec) ->bool{
    ! t.is_valid()
}

#[allow(non_snake_case)]
pub const fn CMP_MACH_TIMESPEC(
    t1: &mach_timespec,
    t2: &mach_timespec
) -> c_ulonglong {
    t1.nsec_diff(t2)
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

#[test]
mod test {
    #[test]
    fn ops_add() {
        mach_timespec::from_nanos()
    }
}
