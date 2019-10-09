#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

/*!

```c
#include "src/Chat.h"
#include "src/GameStructs.h"

#ifdef CC_BUILD_WIN
    #define CC_API __declspec(dllimport)
    #define CC_VAR __declspec(dllimport)
    #define EXPORT __declspec(dllexport)
#else
    #define CC_API
    #define CC_VAR
    #define EXPORT __attribute__((visibility("default")))
#endif

static void TestPlugin_Init(void) {
        String msg = String_FromConst("Hello world!");
        Chat_Add(&msg);
}

EXPORT int Plugin_ApiVersion = 1;
EXPORT struct IGameComponent Plugin_Component = { TestPlugin_Init };
```

```rust
use classicube::*;
use std::{os::raw::c_int, ptr};

#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;

#[no_mangle]
pub static mut Plugin_Component: IGameComponent = IGameComponent {
  /* Called when the game is being loaded. */
  Init: None,
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

#[test]
fn test_linkage() {
  unsafe {
    println!("{:?}", ChatEvents);
  }
}
