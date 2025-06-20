#![allow(clippy::missing_safety_doc)]

use core::mem;

use crate::{
    bindings::{
        cc_bool, cc_string, BlockID, Event_Block, Event_Block_Callback, Event_Chat,
        Event_Chat_Callback, Event_Entry, Event_Entry_Callback, Event_Float, Event_Float_Callback,
        Event_Input, Event_Input_Callback, Event_Int, Event_Int_Callback, Event_PluginMessage,
        Event_PluginMessage_Callback, Event_RawMove, Event_RawMove_Callback, Event_Register,
        Event_String, Event_String_Callback, Event_Unregister, Event_Void, Event_Void_Callback,
        IVec3, InputDevice, Stream,
    },
    std_types::{c_float, c_int, c_void},
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
                    handlers.cast::<Event_Void>(),
                    obj,
                    #[allow(clippy::useless_transmute)]
                    mem::transmute::<[<Event_ $name _Callback>], Event_Void_Callback>(handler),
                )
            }

            pub unsafe fn [<Event_Unregister $func_name>] (
                handlers: *mut [<Event_ $name>],
                obj: *mut c_void,
                handler: [<Event_ $name _Callback>],
            ) {
                Event_Unregister(
                    handlers.cast::<Event_Void>(),
                    obj,
                    #[allow(clippy::useless_transmute)]
                    mem::transmute::<[<Event_ $name _Callback>], Event_Void_Callback>(handler),
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
                handlers: *mut [<Event_ $name>],
                $($arg: $arg_type,)*
            ) {
                let handlers = &mut *handlers;
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
make_raise!(Entry, (stream: *mut Stream, name: *const cc_string));

make_register!(Block);
make_raise!(Block, (coords: IVec3, oldBlock: BlockID, block: BlockID));

make_register!(Chat);
make_raise!(Chat, (msg: *const cc_string, msgType: c_int));

make_register!(Input);
make_raise!(Input, (key: c_int, repeating: cc_bool, device: *mut InputDevice));

make_register!(String);
make_raise!(String, (s: *const cc_string));

make_register!(RawMove);
make_raise!(RawMove, (x_delta: c_float, y_delta: c_float));

make_register!(PluginMessage);
make_raise!(PluginMessage, (channel: u8, data: *mut u8));
