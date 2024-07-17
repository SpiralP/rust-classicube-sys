#![cfg_attr(feature = "no_std", no_std)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "no_std")]
extern crate alloc;

mod bindings;
mod bitmap;
mod chat;
mod command;
mod constants;
mod core;
mod drawer_2d;
mod entity;
mod event;
mod graphics;
mod input;
mod inventory;
mod math;
mod model;
mod packed_col;
mod particle;
pub mod screen;
mod std_types;
mod string;
mod vectors;
mod world;

pub use crate::{
    bindings::*, bitmap::*, chat::*, command::*, constants::*, core::*, drawer_2d::*, entity::*,
    event::*, graphics::*, input::*, inventory::*, math::*, model::*, packed_col::*, particle::*,
    screen::OwnedScreen, string::*, vectors::*, world::*,
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
        println!("{:?}", ::core::ptr::addr_of!(Server) as *const _);
    }
}
