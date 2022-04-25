use crate::std_types::c_char;

use crate::bindings::*;

#[allow(clippy::missing_safety_doc)]
pub unsafe fn Chat_AddRaw(raw: *const c_char) {
    let string = String_FromReadonly(raw);
    Chat_AddOf(&string, MsgType_MSG_TYPE_NORMAL as _);
}
