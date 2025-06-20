use crate::bindings::{cc_int16, cc_int32, cc_uint16, cc_uint8};

pub const UInt8_MaxValue: cc_uint8 = 255_u8;
pub const Int16_MaxValue: cc_int16 = 32767_i16;
pub const UInt16_MaxValue: cc_uint16 = 65535_u16;
pub const Int32_MinValue: cc_int32 = -2_147_483_647_i32 - 1_i32;
pub const Int32_MaxValue: cc_int32 = 2_147_483_647_i32;
