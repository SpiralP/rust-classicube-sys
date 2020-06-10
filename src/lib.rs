#![feature(external_doc)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]
#![doc(include = "../README.md")]

mod bindings;
mod chat;
mod command;
mod entity;
mod event;
mod graphics;
mod gui;
mod input;
mod inventory;
mod model;
mod packed_col;
mod string;
mod vectors;
mod world;

pub use crate::{
    bindings::*, chat::*, command::*, entity::*, event::*, graphics::*, gui::*, input::*,
    inventory::*, model::*, packed_col::*, string::*, vectors::*, world::*,
};

/// On windows, external statics have to be tagged with dllimport,
/// but rust only tags them if you use the #[link] attribute
/// on the exact extern "C" { block } containing the static.
///
/// https://github.com/rust-lang/rust/issues/37403
#[cfg(target_os = "windows")]
#[test]
fn test_dllimport_linking() {
    unsafe {
        println!("{:?}", &Server as *const _);
    }
}
