// static CC_INLINE BlockID World_GetBlock(int x, int y, int z) {
// 	int i = World_Pack(x, y, z);
// 	return (BlockID)((World.Blocks[i] | (World.Blocks2[i] << 8)) & World.IDMask);
// }

use crate::{os::*, World};
use std::{os::raw::c_int, slice};

// /* Unpacka an index into x,y,z (slow!) */
// #define World_Unpack(idx, x, y, z) x = idx % World.Width; z = (idx /
// World.Width) % World.Length; y = (idx / World.Width) / World.Length; /* Packs
// an x,y,z into a single index */ #define World_Pack(x, y, z) (((y) *
// World.Length + (z)) * World.Width + (x))

pub fn World_Pack(x: c_int, y: c_int, z: c_int) -> c_int {
  ((y) * unsafe { World.Length } + (z)) * unsafe { World.Width } + (x)
}

pub fn World_GetBlock(x: c_int, y: c_int, z: c_int) -> BlockID {
  let i = World_Pack(x, y, z) as usize;

  (((World_Blocks()[i] as c_int) | ((World_Blocks2()[i] as c_int) << 8)) & unsafe { World.IDMask })
    as BlockID
}

pub fn World_Blocks() -> &'static mut [BlockRaw] {
  unsafe { slice::from_raw_parts_mut(World.Blocks, World.Volume as _) }
}

pub fn World_Blocks2() -> &'static mut [BlockRaw] {
  unsafe { slice::from_raw_parts_mut(World.Blocks2, World.Volume as _) }
}
