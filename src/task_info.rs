//! This module roughly corresponds to `mach/task_info.h`.

use core::mem::size_of;

use vm_types::{
    integer_t, natural_t,
    mach_vm_address_t, mach_vm_size_t,
};
use message::mach_msg_type_number_t;

use ::libc::c_uint;

pub const TASK_INFO_MAX: c_uint = 1024;
pub const TASK_BASIC_INFO_32: c_uint = 4;
pub const TASK_BASIC2_INFO_32: c_uint = 6;

#[cfg(target_arch = "x86_64")]
pub const TASK_BASIC_INFO_64: c_uint = 5;

#[cfg(target_arch = "aarch64")]
pub const TASK_BASIC_INFO_64: c_uint = 18;

#[cfg(target_bits = "32")]
pub const TASK_BASIC_INFO: c_uint = TASK_BASIC_INFO_32;

#[cfg(target_bits = "64")]
pub const TASK_BASIC_INFO: c_uint = TASK_BASIC_INFO_64;

pub const TASK_EVENTS_INFO: c_uint = 2;
pub const TASK_THREAD_TIMES_INFO: c_uint = 3;
pub const TASK_ABSOLUTETIME_INFO: c_uint = 1;
pub const TASK_KERNELMEMORY_INFO: c_uint = 7;
pub const TASK_SECURITY_TOKEN: c_uint = 13;
pub const TASK_AUDIT_TOKEN: c_uint = 15;
pub const TASK_AFFINITY_TAG_INFO: c_uint = 16;
pub const TASK_DYLD_INFO: c_uint = 17;
pub const TASK_DYLD_ALL_IMAGE_INFO_32: c_uint = 0;
pub const TASK_DYLD_ALL_IMAGE_INFO_64: c_uint = 1;
pub const TASK_EXTMOD_INFO: c_uint = 19;
pub const MACH_TASK_BASIC_INFO: c_uint = 20;
pub const TASK_POWER_INFO: c_uint = 21;
pub const TASK_VM_INFO: c_uint = 22;
pub const TASK_VM_INFO_PURGEABLE: c_uint = 23;
pub const TASK_TRACE_MEMORY_INFO: c_uint = 24;
pub const TASK_WAIT_STATE_INFO: c_uint = 25;
pub const TASK_POWER_INFO_V2: c_uint = 26;
pub const TASK_VM_INFO_PURGEABLE_ACCOUNT: c_uint = 27;
pub const TASK_FLAGS_INFO: c_uint = 28;
pub const TASK_DEBUG_INFO_INTERNAL: c_uint = 29;

pub type task_flavor_t = natural_t;
pub type task_info_t = *mut integer_t;

pub type task_vm_info_t = task_vm_info_rev5_t;
pub type task_vm_info = task_vm_info_t;

pub const TASK_VM_INFO_COUNT: mach_msg_type_number_t =
size_of::<task_vm_info_rev5_t>()/size_of::<natural_t>();

pub const TASK_VM_INFO_REV5_COUNT:mach_msg_type_number_t
    = TASK_VM_INFO_COUNT;

pub const TASK_VM_INFO_REV4_COUNT:mach_msg_type_number_t
    = TASK_VM_INFO_REV5_COUNT - 1;

pub const TASK_VM_INFO_REV3_COUNT:mach_msg_type_number_t
    = TASK_VM_INFO_REV4_COUNT - 2;

pub const TASK_VM_INFO_REV2_COUNT:mach_msg_type_number_t
    = TASK_VM_INFO_REV3_COUNT - 42;

pub const TASK_VM_INFO_REV1_COUNT:mach_msg_type_number_t
    = TASK_VM_INFO_REV2_COUNT - 4;

pub const TASK_VM_INFO_REV0_COUNT:mach_msg_type_number_t
    = TASK_VM_INFO_REV1_COUNT - 2;


// https://github.com/apple/darwin-xnu/blob/2ff845c2e033bd0ff64b5b6aa6063a1f8f65aa32/osfmk/mach/task_info.h#L425
//
/*

#define TASK_VM_INFO_COUNT  ((mach_msg_type_number_t) \
	(sizeof (task_vm_info_data_t) / sizeof (natural_t)))

#define TASK_VM_INFO_REV5_COUNT TASK_VM_INFO_COUNT

// doesn't include decompressions
#define TASK_VM_INFO_REV4_COUNT \
((mach_msg_type_number_t) (TASK_VM_INFO_REV5_COUNT - 1))

// doesn't include limit bytes
#define TASK_VM_INFO_REV3_COUNT \
((mach_msg_type_number_t) (TASK_VM_INFO_REV4_COUNT - 2))

// doesn't include extra ledgers info
#define TASK_VM_INFO_REV2_COUNT \
((mach_msg_type_number_t) (TASK_VM_INFO_REV3_COUNT - 42))

// doesn't include min and max address
#define TASK_VM_INFO_REV1_COUNT \
((mach_msg_type_number_t) (TASK_VM_INFO_REV2_COUNT - 4))

// doesn't include phys_footprint
#define TASK_VM_INFO_REV0_COUNT \
((mach_msg_type_number_t) (TASK_VM_INFO_REV1_COUNT - 2))

*/

#[repr(C, packed(4))]
#[derive(
    Copy, Clone,
    Debug, Default,
    Hash,
    PartialOrd, Ord,
    PartialEq, Eq,
)]
pub struct task_vm_info_rev0_t {
    /// virtual memory size (bytes)
    pub virtual_size:                mach_vm_size_t,

    /// number of memory regions
    pub region_count:                integer_t,

    /// page size
    pub page_size:                   integer_t,

    /// resident memory size (bytes)
    pub resident_size:               mach_vm_size_t,

    /// peak resident size (bytes)
    pub resident_size_peak:          mach_vm_size_t,

    pub device:                      mach_vm_size_t,
    pub device_peak:                 mach_vm_size_t,
    pub internal:                    mach_vm_size_t,
    pub internal_peak:               mach_vm_size_t,
    pub external:                    mach_vm_size_t,
    pub external_peak:               mach_vm_size_t,
    pub reusable:                    mach_vm_size_t,
    pub reusable_peak:               mach_vm_size_t,
    pub purgeable_volatile_pmap:     mach_vm_size_t,
    pub purgeable_volatile_resident: mach_vm_size_t,
    pub purgeable_volatile_virtual:  mach_vm_size_t,
    pub compressed:                  mach_vm_size_t,
    pub compressed_peak:             mach_vm_size_t,
    pub compressed_lifetime:         mach_vm_size_t,
}

#[repr(C, packed(4))]
#[derive(
    Copy, Clone,
    Debug, Default,
    Hash,
    PartialOrd, Ord,
    PartialEq, Eq,
)]
pub struct task_vm_info_rev1_t {
    /// virtual memory size (bytes)
    pub virtual_size:                mach_vm_size_t,

    /// number of memory regions
    pub region_count:                integer_t,

    /// page size
    pub page_size:                   integer_t,

    /// resident memory size (bytes)
    pub resident_size:               mach_vm_size_t,

    /// peak resident size (bytes)
    pub resident_size_peak:          mach_vm_size_t,

    pub device:                      mach_vm_size_t,
    pub device_peak:                 mach_vm_size_t,
    pub internal:                    mach_vm_size_t,
    pub internal_peak:               mach_vm_size_t,
    pub external:                    mach_vm_size_t,
    pub external_peak:               mach_vm_size_t,
    pub reusable:                    mach_vm_size_t,
    pub reusable_peak:               mach_vm_size_t,
    pub purgeable_volatile_pmap:     mach_vm_size_t,
    pub purgeable_volatile_resident: mach_vm_size_t,
    pub purgeable_volatile_virtual:  mach_vm_size_t,
    pub compressed:                  mach_vm_size_t,
    pub compressed_peak:             mach_vm_size_t,
    pub compressed_lifetime:         mach_vm_size_t,

    /// added for rev1
    pub phys_footprint: mach_vm_size_t,
}

#[repr(C, packed(4))]
#[derive(
    Copy, Clone,
    Debug, Default,
    Hash,
    PartialOrd, Ord,
    PartialEq, Eq,
)]
pub struct task_vm_info_rev2_t {
    /// virtual memory size (bytes)
    pub virtual_size:                mach_vm_size_t,

    /// number of memory regions
    pub region_count:                integer_t,

    /// page size
    pub page_size:                   integer_t,

    /// resident memory size (bytes)
    pub resident_size:               mach_vm_size_t,

    /// peak resident size (bytes)
    pub resident_size_peak:          mach_vm_size_t,

    pub device:                      mach_vm_size_t,
    pub device_peak:                 mach_vm_size_t,
    pub internal:                    mach_vm_size_t,
    pub internal_peak:               mach_vm_size_t,
    pub external:                    mach_vm_size_t,
    pub external_peak:               mach_vm_size_t,
    pub reusable:                    mach_vm_size_t,
    pub reusable_peak:               mach_vm_size_t,
    pub purgeable_volatile_pmap:     mach_vm_size_t,
    pub purgeable_volatile_resident: mach_vm_size_t,
    pub purgeable_volatile_virtual:  mach_vm_size_t,
    pub compressed:                  mach_vm_size_t,
    pub compressed_peak:             mach_vm_size_t,
    pub compressed_lifetime:         mach_vm_size_t,

    // added for rev1
    pub phys_footprint: mach_vm_size_t,

    // added for rev2
    pub min_address: mach_vm_address_t,
    pub max_address: mach_vm_address_t,
}

#[repr(C, packed(4))]
#[derive(
    Copy, Clone,
    Debug, Default,
    Hash,
    PartialOrd, Ord,
    PartialEq, Eq,
)]
pub struct task_vm_info_rev3_t {
    /// virtual memory size (bytes)
    pub virtual_size:                mach_vm_size_t,

    /// number of memory regions
    pub region_count:                integer_t,

    /// page size
    pub page_size:                   integer_t,

    /// resident memory size (bytes)
    pub resident_size:               mach_vm_size_t,

    /// peak resident size (bytes)
    pub resident_size_peak:          mach_vm_size_t,

    pub device:                      mach_vm_size_t,
    pub device_peak:                 mach_vm_size_t,
    pub internal:                    mach_vm_size_t,
    pub internal_peak:               mach_vm_size_t,
    pub external:                    mach_vm_size_t,
    pub external_peak:               mach_vm_size_t,
    pub reusable:                    mach_vm_size_t,
    pub reusable_peak:               mach_vm_size_t,
    pub purgeable_volatile_pmap:     mach_vm_size_t,
    pub purgeable_volatile_resident: mach_vm_size_t,
    pub purgeable_volatile_virtual:  mach_vm_size_t,
    pub compressed:                  mach_vm_size_t,
    pub compressed_peak:             mach_vm_size_t,
    pub compressed_lifetime:         mach_vm_size_t,

    // added for rev1
    pub phys_footprint:              mach_vm_size_t,

    // added for rev2
    pub min_address:                 mach_vm_address_t,
    pub max_address:                 mach_vm_address_t,

    // added for rev3
    pub ledger_phys_footprint_peak:                 i64,
    pub ledger_purgeable_nonvolatile:               i64,
    pub ledger_purgeable_novolatile_compressed:     i64,
    pub ledger_purgeable_volatile:                  i64,
    pub ledger_purgeable_volatile_compressed:       i64,
    pub ledger_tag_network_nonvolatile:             i64,
    pub ledger_tag_network_nonvolatile_compressed:  i64,
    pub ledger_tag_network_volatile:                i64,
    pub ledger_tag_network_volatile_compressed:     i64,
    pub ledger_tag_media_footprint:                 i64,
    pub ledger_tag_media_footprint_compressed:      i64,
    pub ledger_tag_media_nofootprint:               i64,
    pub ledger_tag_media_nofootprint_compressed:    i64,
    pub ledger_tag_graphics_footprint:              i64,
    pub ledger_tag_graphics_footprint_compressed:   i64,
    pub ledger_tag_graphics_nofootprint:            i64,
    pub ledger_tag_graphics_nofootprint_compressed: i64,
    pub ledger_tag_neural_footprint:                i64,
    pub ledger_tag_neural_footprint_compressed:     i64,
    pub ledger_tag_neural_nofootprint:              i64,
    pub ledger_tag_neural_nofootprint_compressed:   i64,
}

#[repr(C, packed(4))]
#[derive(
    Copy, Clone,
    Debug, Default,
    Hash,
    PartialOrd, Ord,
    PartialEq, Eq,
)]
pub struct task_vm_info_rev4_t {
    /// virtual memory size (bytes)
    pub virtual_size:                mach_vm_size_t,

    /// number of memory regions
    pub region_count:                integer_t,

    /// page size
    pub page_size:                   integer_t,

    /// resident memory size (bytes)
    pub resident_size:               mach_vm_size_t,

    /// peak resident size (bytes)
    pub resident_size_peak:          mach_vm_size_t,

    pub device:                      mach_vm_size_t,
    pub device_peak:                 mach_vm_size_t,
    pub internal:                    mach_vm_size_t,
    pub internal_peak:               mach_vm_size_t,
    pub external:                    mach_vm_size_t,
    pub external_peak:               mach_vm_size_t,
    pub reusable:                    mach_vm_size_t,
    pub reusable_peak:               mach_vm_size_t,
    pub purgeable_volatile_pmap:     mach_vm_size_t,
    pub purgeable_volatile_resident: mach_vm_size_t,
    pub purgeable_volatile_virtual:  mach_vm_size_t,
    pub compressed:                  mach_vm_size_t,
    pub compressed_peak:             mach_vm_size_t,
    pub compressed_lifetime:         mach_vm_size_t,

    // added for rev1
    pub phys_footprint:              mach_vm_size_t,

    // added for rev2
    pub min_address:                 mach_vm_address_t,
    pub max_address:                 mach_vm_address_t,

    // added for rev3
    pub ledger_phys_footprint_peak:                 i64,
    pub ledger_purgeable_nonvolatile:               i64,
    pub ledger_purgeable_novolatile_compressed:     i64,
    pub ledger_purgeable_volatile:                  i64,
    pub ledger_purgeable_volatile_compressed:       i64,
    pub ledger_tag_network_nonvolatile:             i64,
    pub ledger_tag_network_nonvolatile_compressed:  i64,
    pub ledger_tag_network_volatile:                i64,
    pub ledger_tag_network_volatile_compressed:     i64,
    pub ledger_tag_media_footprint:                 i64,
    pub ledger_tag_media_footprint_compressed:      i64,
    pub ledger_tag_media_nofootprint:               i64,
    pub ledger_tag_media_nofootprint_compressed:    i64,
    pub ledger_tag_graphics_footprint:              i64,
    pub ledger_tag_graphics_footprint_compressed:   i64,
    pub ledger_tag_graphics_nofootprint:            i64,
    pub ledger_tag_graphics_nofootprint_compressed: i64,
    pub ledger_tag_neural_footprint:                i64,
    pub ledger_tag_neural_footprint_compressed:     i64,
    pub ledger_tag_neural_nofootprint:              i64,
    pub ledger_tag_neural_nofootprint_compressed:   i64,

    // added for rev4
    pub limit_bytes_remaining: u64,
}

#[repr(C, packed(4))]
#[derive(
    Copy, Clone,
    Debug, Default,
    Hash,
    PartialOrd, Ord,
    PartialEq, Eq,
)]
pub struct task_vm_info_rev5_t {
    /// virtual memory size (bytes)
    pub virtual_size:                mach_vm_size_t,

    /// number of memory regions
    pub region_count:                integer_t,

    /// page size
    pub page_size:                   integer_t,

    /// resident memory size (bytes)
    pub resident_size:               mach_vm_size_t,

    /// peak resident size (bytes)
    pub resident_size_peak:          mach_vm_size_t,

    pub device:                      mach_vm_size_t,
    pub device_peak:                 mach_vm_size_t,
    pub internal:                    mach_vm_size_t,
    pub internal_peak:               mach_vm_size_t,
    pub external:                    mach_vm_size_t,
    pub external_peak:               mach_vm_size_t,
    pub reusable:                    mach_vm_size_t,
    pub reusable_peak:               mach_vm_size_t,
    pub purgeable_volatile_pmap:     mach_vm_size_t,
    pub purgeable_volatile_resident: mach_vm_size_t,
    pub purgeable_volatile_virtual:  mach_vm_size_t,
    pub compressed:                  mach_vm_size_t,
    pub compressed_peak:             mach_vm_size_t,
    pub compressed_lifetime:         mach_vm_size_t,

    // added for rev1
    pub phys_footprint:              mach_vm_size_t,

    // added for rev2
    pub min_address:                 mach_vm_address_t,
    pub max_address:                 mach_vm_address_t,

    // added for rev3
    pub ledger_phys_footprint_peak:                 i64,
    pub ledger_purgeable_nonvolatile:               i64,
    pub ledger_purgeable_novolatile_compressed:     i64,
    pub ledger_purgeable_volatile:                  i64,
    pub ledger_purgeable_volatile_compressed:       i64,
    pub ledger_tag_network_nonvolatile:             i64,
    pub ledger_tag_network_nonvolatile_compressed:  i64,
    pub ledger_tag_network_volatile:                i64,
    pub ledger_tag_network_volatile_compressed:     i64,
    pub ledger_tag_media_footprint:                 i64,
    pub ledger_tag_media_footprint_compressed:      i64,
    pub ledger_tag_media_nofootprint:               i64,
    pub ledger_tag_media_nofootprint_compressed:    i64,
    pub ledger_tag_graphics_footprint:              i64,
    pub ledger_tag_graphics_footprint_compressed:   i64,
    pub ledger_tag_graphics_nofootprint:            i64,
    pub ledger_tag_graphics_nofootprint_compressed: i64,
    pub ledger_tag_neural_footprint:                i64,
    pub ledger_tag_neural_footprint_compressed:     i64,
    pub ledger_tag_neural_nofootprint:              i64,
    pub ledger_tag_neural_nofootprint_compressed:   i64,

    // added for rev4
    pub limit_bytes_remaining: u64,

    // added for rev5
    pub decompressions: integer_t,
}

#[repr(C, packed(4))]
#[derive(
    Copy, Clone,
    Debug, Default,
    Hash,
    PartialOrd, Ord,
    PartialEq, Eq,
)]
pub struct task_dyld_info {
    pub all_image_info_addr:   mach_vm_address_t,
    pub all_image_info_size:   mach_vm_size_t,
    pub all_image_info_format: integer_t,
}

