//! This module corresponds to `mach/mach_traps.h`.

use crate::ffi::c_int;

use crate::kern_return::kern_return_t;
use crate::port::{mach_port_name_t, mach_port_t};

extern "C" {
    static mach_task_self_: mach_port_t;

    pub fn task_for_pid(
        target_tport: mach_port_name_t,
        pid:          c_int,
        tn:           *mut mach_port_name_t,
    ) -> kern_return_t;
}

#[cfg(feature = "unstable")]
pub mod sync {
    use super::{mach_port_t, mach_task_self_};

    #[cfg(feature = "std")]
    use once_cell::sync::Lazy;
    #[cfg(feature = "std")]
    static _MUTEX_STD: Lazy<parking_lot::Mutex<()>> = Lazy::new(|| { parking_lot::Mutex::new(()) });

    #[cfg(not(feature = "std"))]
    use generic_once_cell::Lazy as GenericLazy;
    #[cfg(not(feature = "std"))]
    use mcslock::{
        raw::spins,
        barging::Mutex as SpinMutex,
        relax::Spin,
    };
    #[cfg(not(feature = "std"))]
    static _MUTEX_MCS: GenericLazy<SpinMutex<(), Spin>, spins::Mutex<()>> = GenericLazy::new(|| { spins::Mutex::new(()) });

    pub fn mach_task_self() -> mach_port_t {
        #[cfg(feature = "std")]
        let _guard = _MUTEX_STD.lock();

        #[cfg(not(feature = "std"))]
        let mut _node = spins::MutexNode::new();
        #[cfg(not(feature = "std"))]
        let _guard = _MUTEX_MCS.lock(&mut _node);

        /// SAFETY: Mutex lock for any access operations
        unsafe { mach_task_self_ }

        // guard dropping here.
    }
}

/// SAFETY? not sure but you can use a thread lock externally, or use [sync::mach_task_self].
pub unsafe fn mach_task_self() -> mach_port_t {
    mach_task_self_
}

/// SAFETY? see [mach_task_self]
pub unsafe fn current_task() -> mach_port_t {
    mach_task_self()
}

#[cfg(test)]
mod tests {
    use crate::port::*;
    use crate::traps::*;

    #[test]
    fn mach_task_self_sanity() {
        unsafe {
            let this_task = mach_task_self();
            assert!(this_task != MACH_PORT_NULL);
            assert!(this_task != MACH_PORT_DEAD);
        }
    }
}

