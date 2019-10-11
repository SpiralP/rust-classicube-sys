use crate::os::*;
use std::{
  borrow::Borrow,
  convert::TryInto,
  ffi::CString,
  marker::PhantomData,
  os::raw::{c_char, c_int},
  slice,
  string::String as StdString,
};

impl String {
  pub fn as_slice(&self) -> &'static [u8] {
    let data = self.buffer as *const u8;
    let len = self.length as usize;
    unsafe { slice::from_raw_parts::<'static>(data, len) }
  }
}

impl ToString for String {
  fn to_string(&self) -> StdString {
    StdString::from_utf8_lossy(self.as_slice()).to_string()
  }
}

pub struct OwnedString<'a> {
  cc_string: String,
  _c_string: CString,
  _phantom: PhantomData<&'a String>,
}

impl<'a> OwnedString<'a> {
  pub fn new<S: Into<Vec<u8>>>(s: S) -> Self {
    let chars = s.into();
    let length = chars.len() as u16;
    let capacity = chars.len() as u16;

    let _c_string = CString::new(chars).unwrap();
    let buffer = _c_string.as_ptr() as *mut c_char;

    Self {
      _c_string,
      cc_string: String {
        buffer,
        length,
        capacity,
      },
      _phantom: PhantomData,
    }
  }

  pub fn as_cc_string(&'a self) -> &'a String {
    &self.cc_string
  }
}

impl<'a> Borrow<String> for OwnedString<'a> {
  fn borrow(&self) -> &String {
    self.as_cc_string()
  }
}

#[test]
fn test_owned_string() {
  let owned_string = OwnedString::new("hello");

  fn use_cc_string<T: Borrow<String>>(s: T) {
    println!("{:?}", s.borrow());
  }

  use_cc_string(owned_string.as_cc_string());

  use_cc_string(owned_string);
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
