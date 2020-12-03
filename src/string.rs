use crate::bindings::{cc_string, cc_uint16, STRING_SIZE};
use std::{
    borrow::Borrow,
    ffi::CString,
    os::raw::{c_char, c_int},
    pin::Pin,
    slice,
};

impl cc_string {
    pub fn as_slice(&self) -> &[u8] {
        let data = self.buffer as *const u8;
        let len = self.length as usize;
        unsafe { slice::from_raw_parts(data, len) }
    }
}

impl ToString for cc_string {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(self.as_slice()).to_string()
    }
}

impl From<cc_string> for String {
    fn from(cc_string: cc_string) -> Self {
        cc_string.to_string()
    }
}

pub struct OwnedString {
    cc_string: cc_string,
    _c_string: Pin<Box<CString>>,
}

impl OwnedString {
    pub fn new<S: Into<Vec<u8>>>(s: S) -> Self {
        let chars = s.into();
        let length = chars.len();
        let capacity = chars.len();

        let mut c_string = Box::pin(CString::new(chars).unwrap());
        let buffer: *const c_char = unsafe { c_string.as_mut().get_unchecked_mut().as_ptr() };

        Self {
            _c_string: c_string,
            cc_string: cc_string {
                buffer: buffer as *mut c_char,
                length: length as cc_uint16,
                capacity: capacity as cc_uint16,
            },
        }
    }

    pub fn as_cc_string(&self) -> &cc_string {
        &self.cc_string
    }
}

impl Borrow<cc_string> for OwnedString {
    fn borrow(&self) -> &cc_string {
        self.as_cc_string()
    }
}

#[test]
fn test_owned_string() {
    let owned_string = OwnedString::new("hello");

    fn use_cc_string<T: Borrow<cc_string>>(s: T) {
        println!("{:?}", s.borrow());
    }

    use_cc_string(owned_string.as_cc_string());

    use_cc_string(owned_string);

    // let s: cc_string = owned_string.into();
}

pub unsafe fn String_Init(buffer: *mut c_char, length: c_int, capacity: c_int) -> cc_string {
    cc_string {
        buffer,
        length: length as _,
        capacity: capacity as _,
    }
}

pub unsafe fn UNSAFE_GetString(data: &[u8]) -> cc_string {
    let mut length = 0;
    for i in (0..STRING_SIZE).rev() {
        let code = data[i as usize];
        if code == b'\0' || code == b' ' {
            continue;
        } else {
            length = i + 1;
            break;
        }
    }

    String_Init(
        data.as_ptr() as *mut c_char,
        length as c_int,
        STRING_SIZE as c_int,
    )
}

#[test]
fn test_get_string() {
    unsafe {
        let mut s =
            b"abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl0000000000000000"
                .to_vec();
        s.resize(STRING_SIZE as usize, 0);
        assert_eq!(
            UNSAFE_GetString(&s).to_string(),
            "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcdefghijkl"
        );
    }
}
