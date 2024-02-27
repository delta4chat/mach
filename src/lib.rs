#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::trivially_copy_pass_by_ref
)]

#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]

// if not a test, then #![no_std]
#![cfg_attr(not(test), no_std)]

// if this is a test, define some useful things
#[cfg(test)]
mod _test_utils {
    // if run tests from `cargo test`, this will force disply to stderr
    #[macro_export]
    macro_rules! p {
        ($($args:tt)*) => {{
            let out = format!( $($args)* );
            let n =
                out.lines()
                .map(|x|{ x.len() })
                .max().unwrap_or(20);

            let mut stderr = std::io::stderr().lock();
            writeln!(stderr, "{}", ".".repeat(n)).expect("cannot print to stderr");
            write!(stderr, "{}", out).expect("cannot print to stderr");
            writeln!(stderr, "{}", ".".repeat(n)).expect("cannot print to stderr");
        }};
    }

    #[macro_export]
    macro_rules! pl {
        ($($args:tt)*) => {{
            let mut s = format!( $($args)* );
            s.push('\n');
            $crate::p!("{}", s);
        }}
    }
}

#[cfg(not(target_vendor="apple"))]
mod _err {
    //compile_error!("mach requires macOS or iOS");
}

#[allow(unused_imports)]
use core::{clone, cmp, default, fmt, hash, marker, mem, option};

pub mod boolean;
pub mod bootstrap;
pub mod clock;
pub mod clock_priv;
pub mod clock_reply;
pub mod clock_types; // TODO: test
pub mod dyld;
pub mod dyld_kernel;
// pub mod error; // TODO
pub mod exc;
pub mod exception_types;
pub mod kern_return;
pub mod loader;
pub mod mach_init;
pub mod mach_port;
pub mod mach_time;
pub mod mach_types;
pub mod memory_object_types;
pub mod message;
pub mod ndr;
pub mod port;
pub mod semaphore;
pub mod structs;
pub mod sync_policy;
pub mod task;
pub mod task_info;
pub mod thread_act;
pub mod thread_policy;
pub mod thread_status;
pub mod time_value;
pub mod traps;
pub mod vm;
pub mod vm_attributes;
pub mod vm_behavior;
pub mod vm_inherit;
pub mod vm_page_size;
pub mod vm_prot;
pub mod vm_purgable;
pub mod vm_region;
pub mod vm_statistics;
pub mod vm_sync;
pub mod vm_types;


