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
use std::{
  convert::TryInto,
  mem,
  os::raw::{c_char, c_int},
  slice,
  string::String as StdString,
};

pub unsafe fn Event_RegisterVoid(
  handlers: *mut Event_Void,
  obj: *mut ::std::os::raw::c_void,
  handler: Event_Void_Callback,
) {
  Event_Register(handlers, obj, handler)
}

pub unsafe fn Event_UnregisterVoid(
  handlers: *mut Event_Void,
  obj: *mut ::std::os::raw::c_void,
  handler: Event_Void_Callback,
) {
  Event_Unregister(handlers, obj, handler)
}

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

pub unsafe fn Event_UnregisterChat(
  handlers: *mut Event_Chat,
  obj: *mut ::std::os::raw::c_void,
  handler: Event_Chat_Callback,
) {
  Event_Unregister(
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

pub unsafe fn String_Init(buffer: *mut c_char, length: c_int, capacity: c_int) -> String {
  String {
    buffer,
    length: length.try_into().unwrap(),
    capacity: capacity.try_into().unwrap(),
  }
}

pub unsafe fn String_FromReadonly(buffer: *const c_char) -> String {
  let len = String_CalcLen(buffer, std::u16::MAX.try_into().unwrap());
  String_Init(buffer as *mut c_char, len, len)
}

pub unsafe fn Chat_AddRaw(raw: *const c_char) {
  let string = String_FromReadonly(raw);
  Chat_AddOf(&string, MsgType_MSG_TYPE_NORMAL);
}

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
  pub static mut KeyEvents: _KeyEventsList;
  pub static mut PointerEvents: _PointerEventsList;
  pub static mut NetEvents: _NetEventsList;
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
