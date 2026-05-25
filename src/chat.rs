use crate::{
    bindings::{Chat_AddOf, MsgType_MSG_TYPE_NORMAL, String_FromReadonly},
    std_types::c_char,
};

#[allow(clippy::missing_safety_doc)]
pub unsafe fn Chat_AddRaw(raw: *const c_char) {
    unsafe {
        let string = String_FromReadonly(raw);
        #[expect(
            clippy::cast_possible_wrap,
            reason = "bindgen enum constant fits in c_int"
        )]
        Chat_AddOf(&raw const string, MsgType_MSG_TYPE_NORMAL as _);
    }
}
