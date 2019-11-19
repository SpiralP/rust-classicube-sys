use crate::os::*;
use std::{
  mem,
  os::raw::{c_int, c_void},
};

pub unsafe fn Event_RegisterVoid(
  handlers: *mut Event_Void,
  obj: *mut c_void,
  handler: Event_Void_Callback,
) {
  Event_Register(handlers, obj, handler)
}

pub unsafe fn Event_UnregisterVoid(
  handlers: *mut Event_Void,
  obj: *mut c_void,
  handler: Event_Void_Callback,
) {
  Event_Unregister(handlers, obj, handler)
}

pub unsafe fn Event_RegisterChat(
  handlers: *mut Event_Chat,
  obj: *mut c_void,
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
  obj: *mut c_void,
  handler: Event_Chat_Callback,
) {
  Event_Unregister(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Chat_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_RegisterInt(
  handlers: *mut Event_Int,
  obj: *mut c_void,
  handler: Event_Int_Callback,
) {
  Event_Register(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Int_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_UnregisterInt(
  handlers: *mut Event_Int,
  obj: *mut c_void,
  handler: Event_Int_Callback,
) {
  Event_Unregister(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Int_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_RegisterInput(
  handlers: *mut Event_Input,
  obj: *mut c_void,
  handler: Event_Input_Callback,
) {
  Event_Register(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Input_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_UnregisterInput(
  handlers: *mut Event_Input,
  obj: *mut c_void,
  handler: Event_Input_Callback,
) {
  Event_Unregister(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Input_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_RegisterFloat(
  handlers: *mut Event_Float,
  obj: *mut c_void,
  handler: Event_Float_Callback,
) {
  Event_Register(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Float_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_UnregisterFloat(
  handlers: *mut Event_Float,
  obj: *mut c_void,
  handler: Event_Float_Callback,
) {
  Event_Unregister(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Float_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_RegisterBlock(
  handlers: *mut Event_Block,
  obj: *mut c_void,
  handler: Event_Block_Callback,
) {
  Event_Register(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Block_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_UnregisterBlock(
  handlers: *mut Event_Block,
  obj: *mut c_void,
  handler: Event_Block_Callback,
) {
  Event_Unregister(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_Block_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_RegisterPointerMove(
  handlers: *mut Event_PointerMove,
  obj: *mut c_void,
  handler: Event_PointerMove_Callback,
) {
  Event_Register(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_PointerMove_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_UnregisterPointerMove(
  handlers: *mut Event_PointerMove,
  obj: *mut c_void,
  handler: Event_PointerMove_Callback,
) {
  Event_Unregister(
    handlers as *mut Event_Void,
    obj,
    mem::transmute::<Event_PointerMove_Callback, Event_Void_Callback>(handler),
  )
}

pub unsafe fn Event_RaiseInput(handlers: &mut Event_Input, key: c_int, repeating: bool) {
  for i in 0..handlers.Count {
    if let Some(f) = handlers.Handlers[i as usize] {
      (f)(
        handlers.Objs[i as usize],
        key,
        if repeating { 1 } else { 0 },
      );
    }
  }
}
