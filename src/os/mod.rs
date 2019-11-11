#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "macos")]
#[inline]
pub(crate) fn as_c_bool(b: bool) -> bool {
  b
}

#[cfg(not(target_os = "macos"))]
#[inline]
pub(crate) fn as_c_bool(b: bool) -> cc_bool {
  if b {
    1
  } else {
    0
  }
}
