use crate::bindings::*;
use std::{
    mem,
    os::raw::{c_float, c_int, c_void},
};

macro_rules! make_register {
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
        make_register!($name, $name);
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

// Raise_ Void, Int, Float are already exported

make_register!(Void);
make_register!(Int);
make_register!(Float);

make_register!(Entry);
make_raise!(Entry, (stream: *mut Stream, name: *const String));

make_register!(Block);
make_raise!(Block, (coords: IVec3, oldBlock: BlockID, block: BlockID));

make_register!(Move, PointerMove);
make_raise!(
    Move,
    PointerMove,
    (idx: c_int, xDelta: c_int, yDelta: c_int)
);

make_register!(Chat);
make_raise!(Chat, (msg: *const String, msgType: c_int));

make_register!(Input);
make_raise!(Input, (key: c_int, repeating: cc_bool));

make_register!(String);
make_raise!(String, (str: *const String));

make_register!(RawMove);
make_raise!(RawMove, (x_delta: c_float, y_delta: c_float));
