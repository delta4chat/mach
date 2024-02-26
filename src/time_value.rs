use crate::vm_types::integer_t;

pub const TIME_MICROS_MAX: integer_t = 1_000_000;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct time_value_t {
    pub seconds:      integer_t,
    pub microseconds: integer_t,
}
pub type time_value = time_value_t;

