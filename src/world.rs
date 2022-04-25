use std::{os::raw::c_int, slice};

use crate::{bindings::*, World};

pub fn World_Pack(x: c_int, y: c_int, z: c_int) -> c_int {
    (y * unsafe { World.Length } + z) * unsafe { World.Width } + x
}

pub fn World_GetBlock(x: c_int, y: c_int, z: c_int) -> BlockID {
    let i = World_Pack(x, y, z) as usize;

    (((World_Blocks()[i] as c_int) | ((World_Blocks2()[i] as c_int) << 8))
        & unsafe { World.IDMask }) as BlockID
}

pub fn World_Blocks() -> &'static mut [BlockRaw] {
    unsafe { slice::from_raw_parts_mut(World.Blocks, World.Volume as _) }
}

pub fn World_Blocks2() -> &'static mut [BlockRaw] {
    unsafe { slice::from_raw_parts_mut(World.Blocks2, World.Volume as _) }
}
