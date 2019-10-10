#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

/*!
```rust
use classicube_sys::*;
use std::{os::raw::c_int, ptr};

extern "C" fn init() {
  // doot
}

#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;

#[no_mangle]
pub static mut Plugin_Component: IGameComponent = IGameComponent {
  /* Called when the game is being loaded. */
  Init: Some(init),
  /* Called when the component is being freed. (e.g. due to game being closed) */
  Free: None,
  /* Called to reset the component's state. (e.g. reconnecting to server) */
  Reset: None,
  /* Called to update the component's state when the user begins loading a new map. */
  OnNewMap: None,
  /* Called to update the component's state when the user has finished loading a new map. */
  OnNewMapLoaded: None,
  /* Next component in linked list of components. */
  next: ptr::null_mut(),
};
```
*/

mod chat;
mod command;
mod event;
mod os;
mod string;

pub use crate::{chat::*, command::*, event::*, os::*, string::*};

// strange fix for these not linking when in generated bindgen
#[link(name = "ClassiCube")]
extern "C" {
  pub static mut EntityEvents: _EntityEventsList;
  pub static mut TabListEvents: _TabListEventsList;
  pub static mut TextureEvents: _TextureEventsList;
  pub static mut GfxEvents: _GfxEventsList;
  pub static mut UserEvents: _UserEventsList;
  pub static mut BlockEvents: _BlockEventsList;
  pub static mut WorldEvents: _WorldEventsList;
  pub static mut ChatEvents: _ChatEventsList;
  pub static mut WindowEvents: _WindowEventsList;
  pub static mut InputEvents: _KeyEventsList;
  pub static mut PointerEvents: _PointerEventsList;
  pub static mut NetEvents: _NetEventsList;

  pub static mut Server: _ServerConnectionData;

  pub static mut TabList: _TabListData;
}

#[test]
fn test_linkage() {
  unsafe {
    println!("{:?}", ChatEvents);
  }
}
