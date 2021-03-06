#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

mod bindings;
mod chat;
mod command;
mod constants;
mod entity;
mod event;
mod graphics;
mod gui;
mod input;
mod inventory;
mod math;
mod model;
mod packed_col;
mod particle;
mod string;
mod vectors;
mod world;

pub use crate::{
    bindings::*, chat::*, command::*, constants::*, entity::*, event::*, graphics::*, gui::*,
    input::*, inventory::*, math::*, model::*, packed_col::*, particle::*, string::*, vectors::*,
    world::*,
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
