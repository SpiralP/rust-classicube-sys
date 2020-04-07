use crate::bindings::*;
use std::{
    mem,
    os::raw::{c_int, c_void},
};

macro_rules! make {
    ($func_name:ident, $name:ident) => {
        paste::item! {
            pub unsafe fn [<Event_Register $func_name>] (
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

            pub unsafe fn [<Event_Unregister $func_name>] (
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

    ($name:ident) => {
        make!($name, $name);
    };
}

macro_rules! make_raise {
    (
        $func_name:ident,
        $name:ident,
        ( $($arg:ident: $arg_type:ty),* )
    ) => {
        paste::item! {
            pub unsafe fn [<Event_Raise $func_name>] (
                handlers: &mut [<Event_ $name>],
                $($arg: $arg_type),*
            ) {
                for i in 0..handlers.Count {
                    if let Some(f) = handlers.Handlers[i as usize] {
                        (f)(
                            handlers.Objs[i as usize],
                            $($arg),*
                        );
                    }
                }
            }
        }
    };

    (
        $name:ident,
        ( $($arg:ident: $arg_type:ty),* )
    ) => {
        make_raise!($name, $name, ( $($arg: $arg_type),* ));
    }
}

make!(Void);
make!(Int);
make!(Float);
make!(Entry);
make!(Block);
make!(Move, PointerMove);
make!(Chat);
make!(Input);
make!(String);

// Void, Int, Float are already exported

make_raise!(Entry, (stream: *mut Stream, name: *const String));
make_raise!(Block, (coords: IVec3, oldBlock: BlockID, block: BlockID));
make_raise!(
    Move,
    PointerMove,
    (idx: c_int, xDelta: c_int, yDelta: c_int)
);
make_raise!(Chat, (msg: *const String, msgType: c_int));
make_raise!(Input, (key: c_int, repeating: cc_bool));
make_raise!(String, (str: *const String));
