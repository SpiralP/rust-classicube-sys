use crate::os::*;
use std::{
  convert::TryInto,
  os::raw::{c_char, c_int},
  slice,
  string::String as StdString,
};

impl ToString for String {
  fn to_string(&self) -> StdString {
    let buffer = self.buffer as *mut u8;
    let length = self.length as usize;

    unsafe { StdString::from_utf8_lossy(slice::from_raw_parts(buffer, length)).to_string() }
  }
}

impl String {
  pub unsafe fn from_string(mut s: StdString) -> Self {
    let buffer = s.as_mut_ptr() as *mut i8;
    let length = s.len() as u16;
    let capacity = s.len() as u16;

    Self {
      buffer,
      length,
      capacity,
    }
  }
}

pub unsafe fn String_Init(buffer: *mut c_char, length: c_int, capacity: c_int) -> String {
  String {
    buffer,
    length: length.try_into().unwrap(),
    capacity: capacity.try_into().unwrap(),
  }
}

pub unsafe fn String_FromReadonly(buffer: *const c_char) -> String {
  let len = String_CalcLen(buffer, std::u16::MAX.try_into().unwrap());
  String_Init(buffer as *mut c_char, len, len)
}
