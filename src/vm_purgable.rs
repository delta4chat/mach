//! This module corresponds to `mach/vm_purgable.h`.

use crate::ffi::c_int;

pub type vm_purgable_t = c_int;

pub const VM_PURGABLE_SET_STATE: vm_purgable_t = 0;
pub const VM_PURGABLE_GET_STATE: vm_purgable_t = 1;

pub const VM_VOLATILE_GROUP_SHIFT:   c_int = 8;
pub const VM_VOLATILE_GROUP_MASK:    c_int = 7 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_DEFAULT: c_int = VM_VOLATILE_GROUP_0;

pub const VM_VOLATILE_GROUP_0: c_int = 0 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_1: c_int = 1 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_2: c_int = 2 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_3: c_int = 3 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_4: c_int = 4 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_5: c_int = 5 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_6: c_int = 6 << VM_VOLATILE_GROUP_SHIFT;
pub const VM_VOLATILE_GROUP_7: c_int = 7 << VM_VOLATILE_GROUP_SHIFT;

pub const VM_PURGABLE_BEHAVIOR_SHIFT: c_int = 6;
pub const VM_PURGABLE_BEHAVIOR_MASK:  c_int = 1 << VM_PURGABLE_BEHAVIOR_SHIFT;
pub const VM_PURGABLE_BEHAVIOR_FIFO:  c_int = 0 << VM_PURGABLE_BEHAVIOR_SHIFT;
pub const VM_PURGABLE_BEHAVIOR_LIFO:  c_int = 1 << VM_PURGABLE_BEHAVIOR_SHIFT;

pub const VM_PURGABLE_ORDERING_SHIFT:    c_int = 5;
pub const VM_PURGABLE_ORDERING_MASK:     c_int = 1 << VM_PURGABLE_ORDERING_SHIFT;
pub const VM_PURGABLE_ORDERING_OBSOLETE: c_int = 1 << VM_PURGABLE_ORDERING_SHIFT;
pub const VM_PURGABLE_ORDERING_NORMAL:   c_int = 0 << VM_PURGABLE_ORDERING_SHIFT;

pub const VM_VOLATILE_ORDER_SHIFT:         c_int = 4;
pub const VM_VOLATILE_ORDER_MASK:          c_int = 1 << VM_VOLATILE_ORDER_SHIFT;
pub const VM_VOLATILE_MAKE_FIRST_IN_GROUP: c_int = 1 << VM_VOLATILE_ORDER_SHIFT;
pub const VM_VOLATILE_MAKE_LAST_IN_GROUP:  c_int = 0 << VM_VOLATILE_ORDER_SHIFT;

pub const VM_PURGABLE_STATE_MIN:   c_int = 0;
pub const VM_PURGABLE_STATE_MAX:   c_int = 3;
pub const VM_PURGABLE_STATE_MASK:  c_int = 3;
pub const VM_PURGABLE_NONVOLATILE: c_int = 0;
pub const VM_PURGABLE_VOLATILE:    c_int = 1;
pub const VM_PURGABLE_EMPTY:       c_int = 2;
pub const VM_PURGABLE_DENY:        c_int = 3;

