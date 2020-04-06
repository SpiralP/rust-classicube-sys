#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]

//! ```rust
//! use classicube_sys::*;
//! use std::{os::raw::c_int, ptr};
//!
//! extern "C" fn init() {
//!   let owned_string = OwnedString::new("hello from rust!");
//!
//!   unsafe {
//!     Chat_Add(owned_string.as_cc_string());
//!   }
//! }
//!
//! #[no_mangle]
//! pub static Plugin_ApiVersion: c_int = 1;
//!
//! #[no_mangle]
//! pub static mut Plugin_Component: IGameComponent = IGameComponent {
//!   /* Called when the game is being loaded. */
//!   Init: Some(init),
//!   /* Called when the component is being freed. (e.g. due to game being closed) */
//!   Free: None,
//!   /* Called to reset the component's state. (e.g. reconnecting to server) */
//!   Reset: None,
//!   /* Called to update the component's state when the user begins loading a new map. */
//!   OnNewMap: None,
//!   /* Called to update the component's state when the user has finished loading a new map. */
//!   OnNewMapLoaded: None,
//!   /* Next component in linked list of components. */
//!   next: ptr::null_mut(),
//! };

mod bindings;
mod chat;
mod command;
mod event;
mod input;
mod packed_col;
mod string;
mod vectors;
mod world;

pub use crate::{
  bindings::*, chat::*, command::*, event::*, input::*, packed_col::*, string::*, vectors::*,
  world::*,
};

/// On windows, external statics have to be tagged with dllimport,
/// but rust only tags them if you use the #[link] attribute
/// on the exact extern "C" { block } containing the static.
///
/// https://github.com/rust-lang/rust/issues/37403
#[test]
fn test_dllimport_linking() {
  unsafe {
    println!("{:?}", &Server as *const _);
  }
}
