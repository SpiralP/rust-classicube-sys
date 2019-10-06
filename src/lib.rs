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
#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;

#[no_mangle]
pub static Plugin_Component: IGameComponent = IGameComponent {
  ...
};

```


*/

mod os;

pub use crate::os::*;
use std::{mem, slice, string::String as StdString};

pub unsafe fn Event_RegisterChat(
  handlers: *mut Event_Chat,
  obj: *mut ::std::os::raw::c_void,
  handler: Event_Chat_Callback,
) {
  Event_Register(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Chat_Callback, Event_Void_Callback>(handler),
  )
}

impl ToString for String {
  fn to_string(&self) -> StdString {
    let buffer = self.buffer as *mut u8;
    let length = self.length as usize;

    unsafe { StdString::from_utf8_lossy(slice::from_raw_parts(buffer, length)).to_string() }
  }
}

impl String {
  pub unsafe fn from_string(mut s: StdString) -> Self {
    let buffer = s.as_mut_ptr() as *mut i8;
    let length = s.len() as u16;
    let capacity = s.len() as u16;

    Self {
      buffer,
      length,
      capacity,
    }
  }
}

#[link(name = "ClassiCube")]
extern "C" {
  pub static mut ChatEvents: _ChatEventsList;
}

#[test]
fn test_linkage() {
  unsafe {
    println!("{:?}", ChatEvents);
  }
}

// #define Event_RegisterChat(handlers,   obj, handler) Event_RegisterMacro(handlers,   obj, handler)

// pub unsafe fn Event_RegisterChat(handlers:*mut Event_Chat,
//   obj: *mut ::std::os::raw::c_void,

//    Event_RegisterMacro(handlers,   obj, handler)
//    }
// pub unsafe fn Event_UnregisterChat(handlers, obj, handler) {
//    Event_UnregisterMacro(handlers, obj, handler)
//    }
