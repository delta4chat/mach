//! This module corresponds to `mach/kern_return.h`.

#![allow(non_camel_case_types)]

use core::ffi::c_int;

// ...Except for this particular type, which is taken from
// 1. `mach/i386/kern_return.h` (x86-based CPUs).
// 2. `mach/arm/kern_return.h`  (arm-based CPUs).
// 
// it is the same type in both header files.
pub type kern_return_t = c_int;

#[derive(
    Copy, Clone,
    Debug, Hash,
    PartialEq, Eq,
)]
pub enum KERN_RETURN {
    KERN_SUCCESS,
    KERN_INVALID_ADDRESS,
    KERN_PROTECTION_FAILURE,
    KERN_NO_SPACE,
    KERN_INVALID_ARGUMENT,
    KERN_FAILURE,
    KERN_RESOURCE_SHORTAGE,
    KERN_NOT_RECEIVER,
    KERN_NO_ACCESS,
    KERN_MEMORY_FAILURE,
    KERN_MEMORY_ERROR,
    KERN_ALREADY_IN_SET,
    KERN_NOT_IN_SET,
    KERN_NAME_EXISTS,
    KERN_ABORTED,
    KERN_INVALID_NAME,
    KERN_INVALID_TASK,
    KERN_INVALID_RIGHT,
    KERN_INVALID_VALUE,
    KERN_UREFS_OVERFLOW,
    KERN_INVALID_CAPABILITY,
    KERN_RIGHT_EXISTS,
    KERN_INVALID_HOST,
    KERN_MEMORY_PRESENT,
    KERN_MEMORY_DATA_MOVED,
    KERN_MEMORY_RESTART_COPY,
    KERN_INVALID_PROCESSOR_SET,
    KERN_POLICY_LIMIT,
    KERN_INVALID_POLICY,
    KERN_INVALID_OBJECT,
    KERN_ALREADY_WAITING,
    KERN_DEFAULT_SET,
    KERN_EXCEPTION_PROTECTED,
    KERN_INVALID_LEDGER,
    KERN_INVALID_MEMORY_CONTROL,
    KERN_INVALID_SECURITY,
    KERN_NOT_DEPRESSED,
    KERN_TERMINATED,
    KERN_LOCK_SET_DESTROYED,
    KERN_LOCK_UNSTABLE,
    KERN_LOCK_OWNED,
    KERN_LOCK_OWNED_SELF,
    KERN_SEMAPHORE_DESTROYED,
    KERN_RPC_SERVER_TERMINATED,
    KERN_RPC_TERMINATE_ORPHAN,
    KERN_RPC_CONTINUE_ORPHAN,
    KERN_NOT_SUPPORTED,
    KERN_NODE_DOWN,
    KERN_NOT_WAITING,
    KERN_OPERATION_TIMED_OUT,
    KERN_CODESIGN_ERROR,
    KERN_POLICY_STATIC,
    KERN_RETURN_MAX,
}
impl Display for KERN_RETURN {
}

impl KERN_RETURN {
    pub const fn stringify(&self) -> String {
        format!("{self:?}")
    }

    pub const fn to_int(&self) -> kern_return_t {
        match self {
            Self::KERN_SUCCESS                => 0,
            Self::KERN_INVALID_ADDRESS        => 1,
            Self::KERN_PROTECTION_FAILURE     => 2,
            Self::KERN_NO_SPACE               => 3,
            Self::KERN_INVALID_ARGUMENT       => 4,
            Self::KERN_FAILURE                => 5,
            Self::KERN_RESOURCE_SHORTAGE      => 6,
            Self::KERN_NOT_RECEIVER           => 7,
            Self::KERN_NO_ACCESS              => 8,
            Self::KERN_MEMORY_FAILURE         => 9,
            Self::KERN_MEMORY_ERROR           => 10,
            Self::KERN_ALREADY_IN_SET         => 11,
            Self::KERN_NOT_IN_SET             => 12,
            Self::KERN_NAME_EXISTS            => 13,
            Self::KERN_ABORTED                => 14,
            Self::KERN_INVALID_NAME           => 15,
            Self::KERN_INVALID_TASK           => 16,
            Self::KERN_INVALID_RIGHT          => 17,
            Self::KERN_INVALID_VALUE          => 18,
            Self::KERN_UREFS_OVERFLOW         => 19,
            Self::KERN_INVALID_CAPABILITY     => 20,
            Self::KERN_RIGHT_EXISTS           => 21,
            Self::KERN_INVALID_HOST           => 22,
            Self::KERN_MEMORY_PRESENT         => 23,
            Self::KERN_MEMORY_DATA_MOVED      => 24,
            Self::KERN_MEMORY_RESTART_COPY    => 25,
            Self::KERN_INVALID_PROCESSOR_SET  => 26,
            Self::KERN_POLICY_LIMIT           => 27,
            Self::KERN_INVALID_POLICY         => 28,
            Self::KERN_INVALID_OBJECT         => 29,
            Self::KERN_ALREADY_WAITING        => 30,
            Self::KERN_DEFAULT_SET            => 31,
            Self::KERN_EXCEPTION_PROTECTED    => 32,
            Self::KERN_INVALID_LEDGER         => 33,
            Self::KERN_INVALID_MEMORY_CONTROL => 34,
            Self::KERN_INVALID_SECURITY       => 35,
            Self::KERN_NOT_DEPRESSED          => 36,
            Self::KERN_TERMINATED             => 37,
            Self::KERN_LOCK_SET_DESTROYED     => 38,
            Self::KERN_LOCK_UNSTABLE          => 39,
            Self::KERN_LOCK_OWNED             => 40,
            Self::KERN_LOCK_OWNED_SELF        => 41,
            Self::KERN_SEMAPHORE_DESTROYED    => 42,
            Self::KERN_RPC_SERVER_TERMINATED  => 43,
            Self::KERN_RPC_TERMINATE_ORPHAN   => 44,
            Self::KERN_RPC_CONTINUE_ORPHAN    => 45,
            Self::KERN_NOT_SUPPORTED          => 46,
            Self::KERN_NODE_DOWN              => 47,
            Self::KERN_NOT_WAITING            => 48,
            Self::KERN_OPERATION_TIMED_OUT    => 49,
            Self::KERN_CODESIGN_ERROR         => 50,
            Self::KERN_POLICY_STATIC          => 51,
            Self::KERN_RETURN_MAX             => 0x100,
        }
    }

    pub const fn from_int(val: kern_return_t) -> Self {
        match val {
            0     => Self::KERN_SUCCESS,
            1     => Self::KERN_INVALID_ADDRESS,
            2     => Self::KERN_PROTECTION_FAILURE,
            3     => Self::KERN_NO_SPACE,
            4     => Self::KERN_INVALID_ARGUMENT,
            5     => Self::KERN_FAILURE,
            6     => Self::KERN_RESOURCE_SHORTAGE,
            7     => Self::KERN_NOT_RECEIVER,
            8     => Self::KERN_NO_ACCESS,
            9     => Self::KERN_MEMORY_FAILURE,
            10    => Self::KERN_MEMORY_ERROR,
            11    => Self::KERN_ALREADY_IN_SET,
            12    => Self::KERN_NOT_IN_SET,
            13    => Self::KERN_NAME_EXISTS,
            14    => Self::KERN_ABORTED,
            15    => Self::KERN_INVALID_NAME,
            16    => Self::KERN_INVALID_TASK,
            17    => Self::KERN_INVALID_RIGHT,
            18    => Self::KERN_INVALID_VALUE,
            19    => Self::KERN_UREFS_OVERFLOW,
            20    => Self::KERN_INVALID_CAPABILITY,
            21    => Self::KERN_RIGHT_EXISTS,
            22    => Self::KERN_INVALID_HOST,
            23    => Self::KERN_MEMORY_PRESENT,
            24    => Self::KERN_MEMORY_DATA_MOVED,
            25    => Self::KERN_MEMORY_RESTART_COPY,
            26    => Self::KERN_INVALID_PROCESSOR_SET,
            27    => Self::KERN_POLICY_LIMIT,
            28    => Self::KERN_INVALID_POLICY,
            29    => Self::KERN_INVALID_OBJECT,
            30    => Self::KERN_ALREADY_WAITING,
            31    => Self::KERN_DEFAULT_SET,
            32    => Self::KERN_EXCEPTION_PROTECTED,
            33    => Self::KERN_INVALID_LEDGER,
            34    => Self::KERN_INVALID_MEMORY_CONTROL,
            35    => Self::KERN_INVALID_SECURITY,
            36    => Self::KERN_NOT_DEPRESSED,
            37    => Self::KERN_TERMINATED,
            38    => Self::KERN_LOCK_SET_DESTROYED,
            39    => Self::KERN_LOCK_UNSTABLE,
            40    => Self::KERN_LOCK_OWNED,
            41    => Self::KERN_LOCK_OWNED_SELF,
            42    => Self::KERN_SEMAPHORE_DESTROYED,
            43    => Self::KERN_RPC_SERVER_TERMINATED,
            44    => Self::KERN_RPC_TERMINATE_ORPHAN,
            45    => Self::KERN_RPC_CONTINUE_ORPHAN,
            46    => Self::KERN_NOT_SUPPORTED,
            47    => Self::KERN_NODE_DOWN,
            48    => Self::KERN_NOT_WAITING,
            49    => Self::KERN_OPERATION_TIMED_OUT,
            50    => Self::KERN_CODESIGN_ERROR,
            51    => Self::KERN_POLICY_STATIC,
            0x100 => Self::KERN_RETURN_MAX,

            _ => {
                panic!("KERN_RETURN code is undefined");
            }
        }
    }
}

macro_rules! _from_int_impl {
    ($t:ty) => {
        impl From<$t> for KERN_RETURN {
            fn from(val: $t) -> KERN_RETURN {
                KERN_RETURN::from_int(val as kern_return_t)
            }
        }
        impl From<KERN_RETURN> for $t {
            fn from(val: KERN_RETURN) -> $t {
                val.to_int() as $t
            }
        }
    }
}


// _from_int_impl!(kern_return_t);
//
// NOTE: this is not required, due to `kern_return_t` always equal to c_int = i32.
// 
//      (even in future, if 32-bit CPUs has deprecated completely...)
//      (so 32-bit no long exists in the IT world, c_int changed to i64, this case will be covered.)
//
//      (in fact it works in all type of integer that Rust-supprted.)
//      (so if one day, may or may not happen: in far future, if all CPUs has been replaced by 128-bit, it also supported.)
//

_from_int_impl!(usize); _from_int_impl!(isize);
_from_int_impl!(u8);    _from_int_impl!(i8);
_from_int_impl!(u16);   _from_int_impl!(i16);
_from_int_impl!(u32);   _from_int_impl!(i32);
_from_int_impl!(u64);   _from_int_impl!(i64);
_from_int_impl!(u128);  _from_int_impl!(i128);

/*
macro_rules! _from_nzint_impl {
    ($t:ty) => {
        impl From<$t> for KERN_RETURN {
            fn from(val: $t) -> KERN_RETURN {
                KERN_RETURN::from_int(val.get() as kern_return_t)
            }
        }
        impl core::convert::TryFrom<KERN_RETURN> for $t {
            type Error = core::num::TryFromIntError;
            fn try_from(val: KERN_RETURN) -> Result<$t, core::num::TryFromIntError> {
                <$t>::try_from( val.into() )
            }
        }
    }
}

use core::num::*;
_from_nzint_impl!(NonZeroUsize); _from_nzint_impl!(NonZeroIsize);
_from_nzint_impl!(NonZeroU8);    _from_nzint_impl!(NonZeroI8);
_from_nzint_impl!(NonZeroU16);   _from_nzint_impl!(NonZeroI16);
_from_nzint_impl!(NonZeroU32);   _from_nzint_impl!(NonZeroI32);
_from_nzint_impl!(NonZeroU64);   _from_nzint_impl!(NonZeroI64);
_from_nzint_impl!(NonZeroU128);  _from_nzint_impl!(NonZeroI128);
*/

/*
macro_rules! _from_aint_impl {
    ($t:ty) => {
        mod _imp_atom {
          use KERN_RETURN;
          use core::sync::atomic::{ $t };

          impl From<$t> for KERN_RETURN {
            fn from(val: $t) -> KERN_RETURN {
              KERN_RETURN::from_int(val.load(core::sync::atomic::Ordering::Relaxed) as _)
            }
          }
          impl From<KERN_RETURN> for $t {
            fn from(val: KERN_RETURN) -> $t {
              <$t>::new(val.to_int() as _)
            }
          }
        }
    }
}

#[cfg(target_has_atomic = "ptr")]
mod _aptr {
    _from_aint_impl!(AtomicUsize); _from_aint_impl!(AtomicIsize);
}

#[cfg(target_has_atomic = "8")]
mod _a8 {

    _from_aint_impl!(AtomicU8);    _from_aint_impl!(AtomicI8);
}

#[cfg(target_has_atomic = "16")]
mod _a16 {
    _from_aint_impl!(AtomicU16);   _from_aint_impl!(AtomicI16);
}

#[cfg(target_has_atomic = "32")]
mod _a32 {
    _from_aint_impl!(AtomicU32);   _from_aint_impl!(AtomicI32);
}

#[cfg(target_has_atomic = "64")]
mod _a64 {
    use core::sync::atomic::{AtomicU64, AtomicI64};
    _from_aint_impl!(AtomicU64);   _from_aint_impl!(AtomicI64);
}

#[cfg(target_has_atomic = "128")]
mod _a128 {
    use core::sync::atomic::{AtomicU8, AtomicI8};
    _from_aint_impl!(AtomicU128);  _from_aint_impl!(AtomicI128);
}
*/

/* this provide compatible for programs that early-written before the OOP changes */

pub const KERN_SUCCESS: kern_return_t =
    KERN_RETURN::KERN_SUCCESS.to_int();

pub const KERN_INVALID_ADDRESS: kern_return_t =
    KERN_RETURN::KERN_INVALID_ADDRESS.to_int();

pub const KERN_PROTECTION_FAILURE: kern_return_t =
    KERN_RETURN::KERN_PROTECTION_FAILURE.to_int();

pub const KERN_NO_SPACE: kern_return_t =
    KERN_RETURN::KERN_NO_SPACE.to_int();

pub const KERN_INVALID_ARGUMENT: kern_return_t =
    KERN_RETURN::KERN_INVALID_ARGUMENT.to_int();

pub const KERN_FAILURE: kern_return_t =
    KERN_RETURN::KERN_FAILURE.to_int();

pub const KERN_RESOURCE_SHORTAGE: kern_return_t =
    KERN_RETURN::KERN_RESOURCE_SHORTAGE.to_int();

pub const KERN_NOT_RECEIVER: kern_return_t =
    KERN_RETURN::KERN_NOT_RECEIVER.to_int();

pub const KERN_NO_ACCESS: kern_return_t =
    KERN_RETURN::KERN_NO_ACCESS.to_int();

pub const KERN_MEMORY_FAILURE: kern_return_t =
    KERN_RETURN::KERN_MEMORY_FAILURE.to_int();

pub const KERN_MEMORY_ERROR: kern_return_t =
    KERN_RETURN::KERN_MEMORY_ERROR.to_int();

pub const KERN_ALREADY_IN_SET: kern_return_t =
    KERN_RETURN::KERN_ALREADY_IN_SET.to_int();

pub const KERN_NOT_IN_SET: kern_return_t =
    KERN_RETURN::KERN_NOT_IN_SET.to_int();

pub const KERN_NAME_EXISTS: kern_return_t =
    KERN_RETURN::KERN_NAME_EXISTS.to_int();

pub const KERN_ABORTED: kern_return_t =
    KERN_RETURN::KERN_ABORTED.to_int();

pub const KERN_INVALID_NAME: kern_return_t =
    KERN_RETURN::KERN_INVALID_NAME.to_int();

pub const KERN_INVALID_TASK: kern_return_t =
    KERN_RETURN::KERN_INVALID_TASK.to_int();

pub const KERN_INVALID_RIGHT: kern_return_t =
    KERN_RETURN::KERN_INVALID_RIGHT.to_int();

pub const KERN_INVALID_VALUE: kern_return_t =
    KERN_RETURN::KERN_INVALID_VALUE.to_int();

pub const KERN_UREFS_OVERFLOW: kern_return_t =
    KERN_RETURN::KERN_UREFS_OVERFLOW.to_int();

pub const KERN_INVALID_CAPABILITY: kern_return_t =
    KERN_RETURN::KERN_INVALID_CAPABILITY.to_int();

pub const KERN_RIGHT_EXISTS: kern_return_t =
    KERN_RETURN::KERN_RIGHT_EXISTS.to_int();

pub const KERN_INVALID_HOST: kern_return_t =
    KERN_RETURN::KERN_INVALID_HOST.to_int();

pub const KERN_MEMORY_PRESENT: kern_return_t =
    KERN_RETURN::KERN_MEMORY_PRESENT.to_int();

pub const KERN_MEMORY_DATA_MOVED: kern_return_t =
    KERN_RETURN::KERN_MEMORY_DATA_MOVED.to_int();

pub const KERN_MEMORY_RESTART_COPY: kern_return_t =
    KERN_RETURN::KERN_MEMORY_RESTART_COPY.to_int();

pub const KERN_INVALID_PROCESSOR_SET: kern_return_t =
    KERN_RETURN::KERN_INVALID_PROCESSOR_SET.to_int();

pub const KERN_POLICY_LIMIT: kern_return_t =
    KERN_RETURN::KERN_POLICY_LIMIT.to_int();

pub const KERN_INVALID_POLICY: kern_return_t =
    KERN_RETURN::KERN_INVALID_POLICY.to_int();

pub const KERN_INVALID_OBJECT: kern_return_t =
    KERN_RETURN::KERN_INVALID_OBJECT.to_int();

pub const KERN_ALREADY_WAITING: kern_return_t =
    KERN_RETURN::KERN_ALREADY_WAITING.to_int();

pub const KERN_DEFAULT_SET: kern_return_t =
    KERN_RETURN::KERN_DEFAULT_SET.to_int();

pub const KERN_EXCEPTION_PROTECTED: kern_return_t =
    KERN_RETURN::KERN_EXCEPTION_PROTECTED.to_int();

pub const KERN_INVALID_LEDGER: kern_return_t =
    KERN_RETURN::KERN_INVALID_LEDGER.to_int();

pub const KERN_INVALID_MEMORY_CONTROL: kern_return_t =
    KERN_RETURN::KERN_INVALID_MEMORY_CONTROL.to_int();

pub const KERN_INVALID_SECURITY: kern_return_t =
    KERN_RETURN::KERN_INVALID_SECURITY.to_int();

pub const KERN_NOT_DEPRESSED: kern_return_t =
    KERN_RETURN::KERN_NOT_DEPRESSED.to_int();

pub const KERN_TERMINATED: kern_return_t =
    KERN_RETURN::KERN_TERMINATED.to_int();

pub const KERN_LOCK_SET_DESTROYED: kern_return_t =
    KERN_RETURN::KERN_LOCK_SET_DESTROYED.to_int();

pub const KERN_LOCK_UNSTABLE: kern_return_t =
    KERN_RETURN::KERN_LOCK_UNSTABLE.to_int();

pub const KERN_LOCK_OWNED: kern_return_t =
    KERN_RETURN::KERN_LOCK_OWNED.to_int();

pub const KERN_LOCK_OWNED_SELF: kern_return_t =
    KERN_RETURN::KERN_LOCK_OWNED_SELF.to_int();

pub const KERN_SEMAPHORE_DESTROYED: kern_return_t =
    KERN_RETURN::KERN_SEMAPHORE_DESTROYED.to_int();

pub const KERN_RPC_SERVER_TERMINATED: kern_return_t =
    KERN_RETURN::KERN_RPC_SERVER_TERMINATED.to_int();

pub const KERN_RPC_TERMINATE_ORPHAN: kern_return_t =
    KERN_RETURN::KERN_RPC_TERMINATE_ORPHAN.to_int();

pub const KERN_RPC_CONTINUE_ORPHAN: kern_return_t =
    KERN_RETURN::KERN_RPC_CONTINUE_ORPHAN.to_int();

pub const KERN_NOT_SUPPORTED: kern_return_t =
    KERN_RETURN::KERN_NOT_SUPPORTED.to_int();

pub const KERN_NODE_DOWN: kern_return_t =
    KERN_RETURN::KERN_NODE_DOWN.to_int();

pub const KERN_NOT_WAITING: kern_return_t =
    KERN_RETURN::KERN_NOT_WAITING.to_int();

pub const KERN_OPERATION_TIMED_OUT: kern_return_t =
    KERN_RETURN::KERN_OPERATION_TIMED_OUT.to_int();

pub const KERN_CODESIGN_ERROR: kern_return_t =
    KERN_RETURN::KERN_CODESIGN_ERROR.to_int();

pub const KERN_POLICY_STATIC: kern_return_t =
    KERN_RETURN::KERN_POLICY_STATIC.to_int();

pub const KERN_RETURN_MAX: kern_return_t =
    KERN_RETURN::KERN_RETURN_MAX.to_int();

