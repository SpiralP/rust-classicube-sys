#![allow(clippy::missing_safety_doc)]

use core::mem;

use crate::{
    bindings::*,
    std_types::{c_float, c_int, c_void},
};

macro_rules! make_register_unregister {
    (
        $(#[$attr:meta])*
        $func_name:ident,
        $name:ident
    ) => {
        paste::item! {
            $(#[$attr])*
            pub unsafe fn [<Event_Register $func_name>] (
                handlers: *mut [<Event_ $name>],
                obj: *mut c_void,
                handler: [<Event_ $name _Callback>],
            ) {
                unsafe {
                    Event_Register(
                        handlers as *mut Event_Void,
                        obj,
                        #[allow(clippy::useless_transmute)]
                        mem::transmute::<[<Event_ $name _Callback>], Event_Void_Callback>(handler),
                    )
                }
            }

            $(#[$attr])*
            pub unsafe fn [<Event_Unregister $func_name>] (
                handlers: *mut [<Event_ $name>],
                obj: *mut c_void,
                handler: [<Event_ $name _Callback>],
            ) {
                unsafe {
                    Event_Unregister(
                        handlers as *mut Event_Void,
                        obj,
                        #[allow(clippy::useless_transmute)]
                        mem::transmute::<[<Event_ $name _Callback>], Event_Void_Callback>(handler),
                    )
                }
            }
        }
    };

    ($name:ident) => {
        make_register_unregister!($name, $name);
    };
}

macro_rules! make_raise {
    (
        $(#[$attr:meta])*
        $name:ident,
        ( $($arg:ident: $arg_type:ty),* )
    ) => {
        paste::item! {
            $(#[$attr])*
            pub unsafe fn [<Event_Raise $name>] (
                handlers: &mut [<Event_ $name>],
                $($arg: $arg_type,)*
            ) {
                for i in 0..handlers.Count {
                    if let Some(f) = handlers.Handlers[i as usize] {
                        unsafe {
                            (f)(
                                handlers.Objs[i as usize],
                                $($arg),*
                            );
                        }
                    }
                }
            }
        }
    };
}

// ClassiCube/src/Event.h

// Event_RaiseVoid, Event_RaiseInt, Event_RaiseFloat are already exported
make_register_unregister!(Void);
make_register_unregister!(Int);
make_register_unregister!(Float);

make_register_unregister!(Entry);
make_raise!(
    /// Calls all registered callbacks for an event which has data stream and name arguments.
    Entry, (stream: *mut Stream, name: *const cc_string)
);

make_register_unregister!(Block);
make_raise!(
    /// Calls all registered callbacks for an event which takes block change arguments.
    /// These are the coordinates/location of the change, block there before, block there now.
    Block, (coords: IVec3, oldBlock: BlockID, block: BlockID)
);

make_register_unregister!(Chat);
make_raise!(
    /// Calls all registered callbacks for an event which has chat message type and contents.
    /// See MsgType enum in Chat.h for what types of messages there are.
    Chat, (msg: *const cc_string, msgType: c_int)
);

make_register_unregister!(Input);
make_raise!(
    /// Calls all registered callbacks for an event which has keyboard key/mouse button.
    /// repeating is whether the key/button was already pressed down. (i.e. user is holding down key)
    Input, (key: c_int, repeating: cc_bool, device: *mut InputDevice)
);

make_register_unregister!(String);
make_raise!(
    /// Calls all registered callbacks for an event which has a string argument.
    String, (s: *const cc_string)
);

make_register_unregister!(RawMove);
make_raise!(
    /// Calls all registered callbacks for an event which has raw pointer movement arguments.
    RawMove, (x_delta: c_float, y_delta: c_float)
);

make_register_unregister!(PadAxis);
make_raise!(
    /// Calls all registered callbacks for an event which has pad axis arguments.
    PadAxis, (port: c_int, axis: c_int, x: c_float, y: c_float)
);

make_register_unregister!(PluginMessage);
make_raise!(
    /// Calls all registered callbacks for an event which has a channel and a 64 byte data argument.
    PluginMessage, (channel: cc_uint8, data: *mut cc_uint8)
);

make_register_unregister!(LightingMode);
make_raise!(
    /// Calls all registered callbacks for an event called when the Lighting_LightingMode is changed
    LightingMode, (oldMode: cc_uint8, fromServer: cc_bool)
);
