use crate::bindings::*;
use std::{
    mem,
    os::raw::{c_int, c_void},
};

macro_rules! make {
    ($name:ident) => {
        paste::item! {
            pub unsafe fn [<Event_Register $name>] (
                handlers: *mut [<Event_ $name>],
                obj: *mut c_void,
                handler: [<Event_ $name _Callback>],
            ) {
                Event_Register(
                    handlers as *mut Event_Void,
                    obj,
                    mem::transmute::<_, Event_Void_Callback>(handler),
                )
            }

            pub unsafe fn [<Event_Unregister $name>] (
                handlers: *mut [<Event_ $name>],
                obj: *mut c_void,
                handler: [<Event_ $name _Callback>],
            ) {
                Event_Unregister(
                    handlers as *mut Event_Void,
                    obj,
                    mem::transmute::<_, Event_Void_Callback>(handler),
                )
            }
        }
    };
}

make!(Void);
make!(Chat);
make!(Int);
make!(Input);
make!(Float);
make!(Block);
make!(PointerMove);
make!(String);

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
