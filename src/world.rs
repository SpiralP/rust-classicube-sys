use core::slice;

use crate::{
    World,
    bindings::{BlockID, BlockRaw},
    std_types::c_int,
};

#[must_use]
pub fn World_Pack(x: c_int, y: c_int, z: c_int) -> c_int {
    (y * unsafe { World.Length } + z) * unsafe { World.Width } + x
}

/// # Panics
///
/// Panics if `(x, y, z)` lies outside the loaded world, producing a negative
/// packed index or a masked id that does not fit in [`BlockID`].
#[must_use]
pub fn World_GetBlock(x: c_int, y: c_int, z: c_int) -> BlockID {
    let i = usize::try_from(World_Pack(x, y, z)).expect("block index out of range");

    let raw = (c_int::from(World_Blocks()[i]) | (c_int::from(World_Blocks2()[i]) << 8))
        & unsafe { World.IDMask };
    BlockID::try_from(raw).expect("masked block id exceeds BlockID range")
}

/// # Panics
///
/// Panics if `World.Volume` is negative.
#[must_use]
pub fn World_Blocks() -> &'static mut [BlockRaw] {
    let volume = usize::try_from(unsafe { World.Volume }).expect("world volume out of range");
    unsafe { slice::from_raw_parts_mut(World.Blocks, volume) }
}

/// # Panics
///
/// Panics if `World.Volume` is negative.
#[must_use]
pub fn World_Blocks2() -> &'static mut [BlockRaw] {
    let volume = usize::try_from(unsafe { World.Volume }).expect("world volume out of range");
    unsafe { slice::from_raw_parts_mut(World.Blocks2, volume) }
}
