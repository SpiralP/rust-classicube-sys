use crate::bindings::{String as CCString, String_CalcLen};
use std::{
    borrow::Borrow,
    ffi::CString,
    os::raw::{c_char, c_int},
    slice,
};

impl CCString {
    pub fn as_slice(&self) -> &'static [u8] {
        let data = self.buffer as *const u8;
        let len = self.length as usize;
        unsafe { slice::from_raw_parts::<'static>(data, len) }
    }
}

impl ToString for CCString {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(self.as_slice()).to_string()
    }
}

impl From<CCString> for String {
    fn from(cc_string: CCString) -> Self {
        cc_string.to_string()
    }
}

pub struct OwnedString {
    cc_string: CCString,
    _c_string: CString,
}

impl OwnedString {
    pub fn new<S: Into<Vec<u8>>>(s: S) -> Self {
        let chars = s.into();
        let length = chars.len() as u16;
        let capacity = chars.len() as u16;

        let _c_string = CString::new(chars).unwrap();
        let buffer = _c_string.as_ptr() as *mut c_char;

        Self {
            _c_string,
            cc_string: CCString {
                buffer,
                length,
                capacity,
            },
        }
    }

    pub fn as_cc_string(&self) -> &CCString {
        &self.cc_string
    }
}

impl Borrow<CCString> for OwnedString {
    fn borrow(&self) -> &CCString {
        self.as_cc_string()
    }
}

#[test]
fn test_owned_string() {
    let owned_string = OwnedString::new("hello");

    fn use_cc_string<T: Borrow<CCString>>(s: T) {
        println!("{:?}", s.borrow());
    }

    use_cc_string(owned_string.as_cc_string());

    use_cc_string(owned_string);

    // let s: CCString = owned_string.into();
}

pub unsafe fn String_Init(buffer: *mut c_char, length: c_int, capacity: c_int) -> CCString {
    CCString {
        buffer,
        length: length as _,
        capacity: capacity as _,
    }
}

pub unsafe fn String_FromReadonly(buffer: *const c_char) -> CCString {
    let len = String_CalcLen(buffer, std::u16::MAX as _);
    String_Init(buffer as *mut c_char, len, len)
}
