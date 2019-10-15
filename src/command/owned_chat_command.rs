use crate::os::as_c_bool;
use crate::ChatCommand;
use arrayvec::ArrayVec;
use std::{ffi::CString, os::raw::c_int, ptr};

pub struct OwnedChatCommand {
  pub name: CString,
  pub help: Vec<CString>,
  pub command: ChatCommand,
}

impl OwnedChatCommand {
  pub fn new(
    name: &'static str,
    execute: unsafe extern "C" fn(args: *const crate::String, argsCount: c_int),
    singleplayer_only: bool,
    mut help: Vec<&'static str>,
  ) -> Self {
    let name = CString::new(name).unwrap();

    let help: Vec<CString> = help.drain(..).map(|s| CString::new(s).unwrap()).collect();

    let command = ChatCommand {
      Name: name.as_ptr(),
      Execute: Some(execute),
      SingleplayerOnly: as_c_bool(singleplayer_only),
      Help: {
        let mut array: ArrayVec<[*const ::std::os::raw::c_char; 5usize]> =
          help.iter().map(|cstr| cstr.as_ptr()).collect();

        while !array.is_full() {
          array.push(ptr::null());
        }

        array.into_inner().unwrap()
      },
      next: ptr::null_mut(),
    };

    Self {
      name,
      help,
      command,
    }
  }
}
