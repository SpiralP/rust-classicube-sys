use core::slice;

use crate::{
    bindings::{BlockID, BlockRaw},
    std_types::c_int,
    World,
};

#[must_use]
pub fn World_Pack(x: c_int, y: c_int, z: c_int) -> c_int {
    (y * unsafe { World.Length } + z) * unsafe { World.Width } + x
}

#[must_use]
pub fn World_GetBlock(x: c_int, y: c_int, z: c_int) -> BlockID {
    let i = World_Pack(x, y, z) as usize;

    ((c_int::from(World_Blocks()[i]) | (c_int::from(World_Blocks2()[i]) << 8))
        & unsafe { World.IDMask }) as BlockID
}

pub fn World_Blocks() -> &'static mut [BlockRaw] {
    unsafe { slice::from_raw_parts_mut(World.Blocks, World.Volume as _) }
}

pub fn World_Blocks2() -> &'static mut [BlockRaw] {
    unsafe { slice::from_raw_parts_mut(World.Blocks2, World.Volume as _) }
}
