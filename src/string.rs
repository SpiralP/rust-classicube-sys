use core::{
    borrow::Borrow,
    ffi::CStr,
    fmt::{self, Display},
    slice,
};

use crate::{
    bindings::{cc_codepoint, cc_string, cc_uint16, cc_unichar, STRING_SIZE},
    std_types::{c_char, c_int, Box, CString, String, ToString, Vec},
};

impl cc_string {
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        let len = self.length as usize;
        let data = self.buffer as *const u8;
        if len == 0 || data.is_null() {
            return &[];
        }
        unsafe { slice::from_raw_parts(data, len) }
    }
}

impl Display for cc_string {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::from_utf16_lossy(
            &self
                .as_slice()
                .iter()
                .map(|c| Convert_CP437ToUnicode(*c))
                .collect::<Vec<_>>(),
        )
        .fmt(f)
    }
}

impl From<cc_string> for String {
    fn from(cc_string: cc_string) -> Self {
        cc_string.to_string()
    }
}

pub struct OwnedString {
    cc_string: cc_string,

    #[allow(dead_code)]
    c_str: Box<CStr>,
}

impl OwnedString {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new<S: Into<String>>(s: S) -> Self {
        let bytes = s
            .into()
            // TODO is chars() "codepoints" in cc's definition?
            .chars()
            .map(|c| c as cc_codepoint)
            .map(Convert_CodepointToCP437)
            .collect::<Vec<_>>();
        let length = bytes.len();
        let capacity = bytes.len();

        let c_str = CString::new(bytes).unwrap().into_boxed_c_str();
        let buffer: *const c_char = c_str.as_ptr();

        Self {
            c_str,
            cc_string: cc_string {
                buffer: buffer.cast_mut(),
                length: length as cc_uint16,
                capacity: capacity as cc_uint16,
            },
        }
    }

    #[must_use]
    pub fn as_cc_string(&self) -> &cc_string {
        &self.cc_string
    }

    /// # Safety
    ///
    /// The `OwnedString` needs to live longer than the `cc_string` return here.
    #[must_use]
    pub unsafe fn get_cc_string(&self) -> cc_string {
        cc_string { ..self.cc_string }
    }
}

impl Borrow<cc_string> for OwnedString {
    fn borrow(&self) -> &cc_string {
        self.as_cc_string()
    }
}

#[test]
fn test_owned_string() {
    fn use_cc_string<T: Borrow<cc_string>>(s: T) {
        #[cfg(not(feature = "no_std"))]
        {
            println!("{:?}", s.borrow());
        }
    }

    let owned_string = OwnedString::new("hello");

    use_cc_string(owned_string.as_cc_string());

    use_cc_string(owned_string);

    // let s: cc_string = owned_string.into();
}

/// # Safety
///
/// The `buffer` needs to live longer than the `cc_string`.
pub unsafe fn String_Init(buffer: *mut c_char, length: c_int, capacity: c_int) -> cc_string {
    cc_string {
        buffer,
        length: length as _,
        capacity: capacity as _,
    }
}

#[allow(clippy::missing_safety_doc)]
#[must_use]
pub unsafe fn UNSAFE_GetString(data: &[u8]) -> cc_string {
    let mut length = 0;
    for i in (0..STRING_SIZE).rev() {
        let code = data[i as usize];
        if code == b'\0' || code == b' ' {
            continue;
        }
        length = i + 1;
        break;
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

pub const controlChars: &[cc_unichar] = &[
    0x0000, 0x263A, 0x263B, 0x2665, 0x2666, 0x2663, 0x2660, 0x2022, 0x25D8, 0x25CB, 0x25D9, 0x2642,
    0x2640, 0x266A, 0x266B, 0x263C, 0x25BA, 0x25C4, 0x2195, 0x203C, 0x00B6, 0x00A7, 0x25AC, 0x21A8,
    0x2191, 0x2193, 0x2192, 0x2190, 0x221F, 0x2194, 0x25B2, 0x25BC,
];

pub const extendedChars: &[cc_unichar] = &[
    0x2302, 0x00C7, 0x00FC, 0x00E9, 0x00E2, 0x00E4, 0x00E0, 0x00E5, 0x00E7, 0x00EA, 0x00EB, 0x00E8,
    0x00EF, 0x00EE, 0x00EC, 0x00C4, 0x00C5, 0x00C9, 0x00E6, 0x00C6, 0x00F4, 0x00F6, 0x00F2, 0x00FB,
    0x00F9, 0x00FF, 0x00D6, 0x00DC, 0x00A2, 0x00A3, 0x00A5, 0x20A7, 0x0192, 0x00E1, 0x00ED, 0x00F3,
    0x00FA, 0x00F1, 0x00D1, 0x00AA, 0x00BA, 0x00BF, 0x2310, 0x00AC, 0x00BD, 0x00BC, 0x00A1, 0x00AB,
    0x00BB, 0x2591, 0x2592, 0x2593, 0x2502, 0x2524, 0x2561, 0x2562, 0x2556, 0x2555, 0x2563, 0x2551,
    0x2557, 0x255D, 0x255C, 0x255B, 0x2510, 0x2514, 0x2534, 0x252C, 0x251C, 0x2500, 0x253C, 0x255E,
    0x255F, 0x255A, 0x2554, 0x2569, 0x2566, 0x2560, 0x2550, 0x256C, 0x2567, 0x2568, 0x2564, 0x2565,
    0x2559, 0x2558, 0x2552, 0x2553, 0x256B, 0x256A, 0x2518, 0x250C, 0x2588, 0x2584, 0x258C, 0x2590,
    0x2580, 0x03B1, 0x00DF, 0x0393, 0x03C0, 0x03A3, 0x03C3, 0x00B5, 0x03C4, 0x03A6, 0x0398, 0x03A9,
    0x03B4, 0x221E, 0x03C6, 0x03B5, 0x2229, 0x2261, 0x00B1, 0x2265, 0x2264, 0x2320, 0x2321, 0x00F7,
    0x2248, 0x00B0, 0x2219, 0x00B7, 0x221A, 0x207F, 0x00B2, 0x25A0, 0x00A0,
];

#[must_use]
pub fn Convert_CP437ToUnicode(raw: u8) -> cc_unichar {
    if raw < 0x20 {
        controlChars[raw as usize]
    } else if raw < 0x7F {
        cc_unichar::from(raw)
    } else {
        extendedChars[raw as usize - 0x7F]
    }
}

#[must_use]
pub fn Convert_CodepointToCP437(cp: cc_codepoint) -> u8 {
    let mut c: u8 = 0;
    Convert_TryCodepointToCP437(cp, &mut c);
    c
}

fn ReduceEmoji(cp: cc_codepoint) -> cc_codepoint {
    if cp == 0x1F31E {
        return 0x263C;
    }
    if cp == 0x1F3B5 {
        return 0x266B;
    }
    if cp == 0x1F642 {
        return 0x263A;
    }

    if cp == 0x1F600 || cp == 0x1F601 || cp == 0x1F603 {
        return 0x263A;
    }
    if cp == 0x1F604 || cp == 0x1F606 || cp == 0x1F60A {
        return 0x263A;
    }
    cp
}

fn Convert_TryCodepointToCP437(mut cp: cc_codepoint, c: &mut u8) -> bool {
    if (0x20..0x7F).contains(&cp) {
        *c = cp as u8;
        return true;
    }
    if cp >= 0x1F000 {
        cp = ReduceEmoji(cp);
    }

    for (i, &chr) in controlChars.iter().enumerate() {
        if cc_codepoint::from(chr) == cp {
            *c = i as u8;
            return true;
        }
    }

    for (i, &chr) in extendedChars.iter().enumerate() {
        if cc_codepoint::from(chr) == cp {
            *c = (i + 0x7F) as u8;
            return true;
        }
    }

    *c = b'?';
    false
}

#[test]
fn test_cp_437_conversion() {
    let bytes: &[u8] = &[97, 236, 236]; // "a∞∞"

    let c_str = CString::new(bytes).unwrap();
    let a = cc_string {
        buffer: c_str.as_ptr().cast_mut(),
        length: bytes.len() as cc_uint16,
        capacity: bytes.len() as cc_uint16,
    };
    assert_eq!(a.to_string(), "a∞∞");

    let str = "a∞∞";
    let s = OwnedString::new(str);
    unsafe {
        assert_eq!(
            slice::from_raw_parts(
                s.as_cc_string().buffer as *const u8,
                s.as_cc_string().length as usize
            ),
            bytes
        );
    }
}
