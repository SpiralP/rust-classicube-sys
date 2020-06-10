use crate::bindings::*;
use std::os::raw::c_int;

/// Gets the block at the nth index in the current hotbar.
pub fn Inventory_Get(idx: c_int) -> BlockID {
    unsafe { Inventory.Table[(Inventory.Offset + idx) as usize] }
}

/// Sets the block at the nth index in the current hotbar.
pub fn Inventory_Set(idx: c_int, block: BlockID) {
    unsafe {
        Inventory.Table[(Inventory.Offset + idx) as usize] = block;
    }
}

/// Gets the currently selected block.
pub fn Inventory_SelectedBlock() -> BlockID {
    unsafe { Inventory_Get(Inventory.SelectedIndex) }
}
