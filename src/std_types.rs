#[cfg(not(feature = "no_std"))]
mod inner {
    pub use std::{
        boxed::Box,
        ffi::CString,
        os::raw::*,
        string::{String, ToString},
        vec,
        vec::Vec,
    };

    pub fn cosf(a: f32) -> f32 {
        a.cos()
    }
    pub fn sinf(a: f32) -> f32 {
        a.sin()
    }
    pub fn floorf(a: f32) -> f32 {
        a.floor()
    }
    pub fn sqrtf(a: f32) -> f32 {
        a.sqrt()
    }
}

#[cfg(feature = "no_std")]
mod inner {
    pub use alloc::{
        boxed::Box,
        string::{String, ToString},
        vec,
        vec::Vec,
    };

    pub use cstr_core::CString;
    pub use libc::*;
    pub use libm::*;
}

pub use inner::*;
