use crate::{
    bindings::{cc_string, Commands_Register},
    ChatCommand, COMMAND_FLAG_SINGLEPLAYER_ONLY,
};
use std::{ffi::CString, os::raw::c_int, pin::Pin, ptr};

pub struct OwnedChatCommand {
    pub name: Pin<Box<CString>>,
    pub help: Vec<Pin<Box<CString>>>,
    pub command: Pin<Box<ChatCommand>>,
}

impl OwnedChatCommand {
    pub fn new(
        name: &str,
        execute: unsafe extern "C" fn(args: *const cc_string, argsCount: c_int),
        singleplayer_only: bool,
        mut help: Vec<&str>,
    ) -> Self {
        let name = Box::pin(CString::new(name).unwrap());

        let help: Vec<Pin<Box<CString>>> = help
            .drain(..)
            .map(|s| Box::pin(CString::new(s).unwrap()))
            .collect();

        let help_array = [
            help.get(0).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
            help.get(1).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
            help.get(2).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
            help.get(3).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
            help.get(4).map(|cs| cs.as_ptr()).unwrap_or(ptr::null()),
        ];

        let command = Box::pin(ChatCommand {
            name: name.as_ptr(),
            Execute: Some(execute),
            flags: if singleplayer_only {
                COMMAND_FLAG_SINGLEPLAYER_ONLY as _
            } else {
                0
            },
            help: help_array,
            next: ptr::null_mut(),
        });

        Self {
            name,
            help,
            command,
        }
    }

    pub fn register(&mut self) {
        let OwnedChatCommand { command, .. } = self;

        unsafe {
            Commands_Register(command.as_mut().get_unchecked_mut());
        }
    }
}

// #[test]
// fn test_owned_chat_command() {
//     extern "C" fn c_command_callback(_args: *const crate::String, _args_count: c_int) {}
//     let mut cmd = OwnedChatCommand::new("Roll", c_command_callback, false, vec![]);
//     cmd.as_mut().register();
// }
