use crate::{bindings::*, std_types::c_char};

#[allow(clippy::missing_safety_doc)]
pub unsafe fn Chat_AddRaw(raw: *const c_char) {
    unsafe {
        let string = String_FromReadonly(raw);
        Chat_AddOf(&raw const string, MsgType_MSG_TYPE_NORMAL as _);
    }
}
