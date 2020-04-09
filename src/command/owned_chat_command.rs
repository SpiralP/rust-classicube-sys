use crate::{bindings::Commands_Register, ChatCommand};
use pin_project::{pin_project, project};
use std::{ffi::CString, os::raw::c_int, pin::Pin, ptr};

#[pin_project]
pub struct OwnedChatCommand {
    #[pin]
    pub name: CString,
    #[pin]
    pub help: Vec<CString>,
    #[pin]
    pub command: ChatCommand,
}

impl OwnedChatCommand {
    pub fn new(
        name: &str,
        execute: unsafe extern "C" fn(args: *const crate::String, argsCount: c_int),
        singleplayer_only: bool,
        mut help: Vec<&str>,
    ) -> Pin<Box<OwnedChatCommand>> {
        let name = CString::new(name).unwrap();

        let help: Vec<CString> = help.drain(..).map(|s| CString::new(s).unwrap()).collect();

        let help_array = [
            help.get(0).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
            help.get(1).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
            help.get(2).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
            help.get(3).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
            help.get(4).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
        ];

        let command = ChatCommand {
            Name: name.as_ptr(),
            Execute: Some(execute),
            SingleplayerOnly: if singleplayer_only { 1 } else { 0 },
            Help: help_array,
            next: ptr::null_mut(),
        };

        Box::pin(Self {
            name,
            help,
            command,
        })
    }

    #[project]
    pub fn register(self: Pin<&mut OwnedChatCommand>) {
        #[project]
        let OwnedChatCommand { mut command, .. } = self.project();

        unsafe {
            Commands_Register(command.as_mut().get_unchecked_mut());
        }
    }
}

#[ignore]
#[test]
fn test_owned_chat_command() {
    extern "C" fn c_command_callback(_args: *const crate::String, _args_count: c_int) {}

    let mut cmd = OwnedChatCommand::new("Roll", c_command_callback, false, vec![]);

    cmd.as_mut().register();
}
