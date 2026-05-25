use core::{ffi::CStr, ptr};

use crate::{
    COMMAND_FLAG_SINGLEPLAYER_ONLY, ChatCommand,
    bindings::{Commands_Register, cc_string},
    std_types::{Box, CString, Vec, c_int},
};

pub struct OwnedChatCommand {
    pub name: Box<CStr>,
    pub help: Vec<Box<CStr>>,
    pub command: Box<ChatCommand>,
}

impl OwnedChatCommand {
    /// # Panics
    ///
    /// Panics if `name` or any entry in `help` contains an interior NUL byte.
    pub fn new(
        name: &str,
        execute: unsafe extern "C" fn(args: *const cc_string, argsCount: c_int),
        singleplayer_only: bool,
        mut help: Vec<&str>,
    ) -> Self {
        let name = CString::new(name).unwrap().into_boxed_c_str();

        let help: Vec<Box<CStr>> = help
            .drain(..)
            .map(|s| CString::new(s).unwrap().into_boxed_c_str())
            .collect();

        let help_array = [
            #[expect(
                clippy::get_first,
                reason = "consistent indexing across the 5-slot array"
            )]
            help.get(0).map_or(ptr::null(), |cs| cs.as_ptr()),
            help.get(1).map_or(ptr::null(), |cs| cs.as_ptr()),
            help.get(2).map_or(ptr::null(), |cs| cs.as_ptr()),
            help.get(3).map_or(ptr::null(), |cs| cs.as_ptr()),
            help.get(4).map_or(ptr::null(), |cs| cs.as_ptr()),
        ];

        let command = Box::new(ChatCommand {
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
            Commands_Register(command.as_mut());
        }
    }
}

// #[test]
// fn test_owned_chat_command() {
//     extern "C" fn c_command_callback(_args: *const crate::String, _args_count: c_int) {}
//     let mut cmd = OwnedChatCommand::new("Roll", c_command_callback, false, vec![]);
//     cmd.as_mut().register();
// }
