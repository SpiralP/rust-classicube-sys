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

// #[cfg_attr(target_os = "linux", link(name = "pcap"))]
// #[cfg_attr(target_os = "windows", link(name = "wpcap"))]
// #[cfg_attr(target_os = "macos", link(name = "pcap"))]
// extern "C" {}
