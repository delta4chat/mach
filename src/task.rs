//! This module corresponds to `mach/task.h`.

use core::ffi::c_int;

use crate::boolean::boolean_t;
use crate::vm_types::{natual_t, integer_t};
use crate::kern_return::kern_return_t;
use crate::mach_types::{
    task_name_t,
    task_t,
    thread_act_array_t,
    thread_act_t,
};
use crate::message::mach_msg_type_number_t;
use crate::port::{mach_port_array_t, mach_port_t};
use crate::task_info::{task_flavor_t, task_info_t};
use crate::thread_status::{thread_state_flavor_t, thread_state_t},

pub type task_special_port_t  = c_int;

pub type task_policy_set_t    = mach_port_t;
pub type task_policy_get_t    = mach_port_t;
pub type task_policy_flavor_t = natural_t;
pub type task_policy_t        = *mut integer_t;

pub const TASK_KERNEL_PORT:    task_special_port_t = 1;
pub const TASK_HOST_PORT:      task_special_port_t = 2;
pub const TASK_NAME_PORT:      task_special_port_t = 3;
pub const TASK_BOOTSTRAP_PORT: task_special_port_t = 4;

extern "C" {
    pub fn task_resume (target_task: task_t) -> kern_return_t;
    pub fn task_suspend(target_task: task_t) -> kern_return_t;

    pub fn task_get_special_port(
        task:         task_t,
        which_port:   task_special_port_t,
        special_port: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn task_set_special_port(
        task:         task_t,
        which_port:   c_int,
        special_port: mach_port_t
    ) -> kern_return_t;

    pub fn mach_ports_lookup(
        target_task:      task_t,
        init_port_set:    *mut mach_port_array_t,
        init_port_setCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn mach_ports_register(
        target_task:      task_t,
        init_port_set:    mach_port_array_t,
        init_port_setCnt: mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn task_threads(
        target_task:  task_t,
        act_list:     *mut thread_act_array_t,
        act_list_cnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn thread_create_running(
        parent_task:  task_t,
        flavor:       thread_state_flavor_t,
        new_state:    thread_state_t,
        new_stateCnt: mach_msg_type_number_t,
        child_act:    *mut thread_act_t,
    ) -> kern_return_t;

    pub fn task_info(
        target_task:      task_name_t,
        flavor:           task_flavor_t,
        task_info_out:    task_info_t,
        task_info_outCnt: *mut mach_msg_type_number_t,
    ) -> kern_return_t;
    pub fn task_set_info(
        target_task:     task_t,
        flavor:          task_flavor_t,
        task_info_in:    task_info_t,
        task_info_inCnt: mach_msg_type_number_t,
    ) -> kern_return_t;

    pub fn task_policy_get(
        task:           task_policy_get_t,
        flavor:         task_policy_flavor_t,
        policy_info:    task_policy_t,
        policy_infoCnt: *mut mach_msg_type_number_t,
        get_default:    *mut boolean_t,
    ) -> kern_return_t;
    pub fn task_policy_set(
        task:           task_policy_set_t,
        flavor:         task_policy_flavor_t,
        policy_info:    task_policy_t,
        policy_infoCnt: mach_msg_type_number_t,
    ) -> kern_return_t;
}

