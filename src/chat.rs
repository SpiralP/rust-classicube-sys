use crate::{bindings::*, string::*};
use std::{convert::TryInto, os::raw::c_char};

pub unsafe fn Chat_AddRaw(raw: *const c_char) {
  let string = String_FromReadonly(raw);
  Chat_AddOf(&string, MsgType_MSG_TYPE_NORMAL.try_into().unwrap());
}
