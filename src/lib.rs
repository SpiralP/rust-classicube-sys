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

mod chat;
mod event;
mod os;
mod string;

pub use crate::{chat::*, event::*, os::*, string::*};

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
