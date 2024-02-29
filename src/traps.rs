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

#[cfg(feature = "alloc")]
pub mod sync {
    extern crate alloc;
    use alloc::sync::Arc;

    #[cfg(feature = "std")]
    use parking_lot;
    #[cfg(feature = "std")]
    static _MUTEX_STD: Arc<parking_lot::Mutex<()>> = Arc::new(Mutex::new(()));

    #[cfg(not(feature = "std"))]
    use mcslock::raw::spins;
    #[cfg(not(feature = "std"))]
    static _MUTEX_MCS: Arc<spins::Mutex<()>> = Arc::new(spins::Mutex::new(()));

    pub fn mach_task_self() -> mach_port_t {
        #[cfg(feature = "std")]
        let _guard = _MUTEX_STD.lock();

        #[cfg(not(feature = "std"))]
        let _guard = _MUTEX_MCS.lock(&mut spins::Node::new());

        /// SAFETY: Mutex lock for any access operations
        unsafe { mach_task_self_ }

        // guard dropping here.
    }
}

pub unsafe fn mach_task_self() -> mach_port_t {
    mach_task_self_
}

pub fn current_task() -> mach_port_t {
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

