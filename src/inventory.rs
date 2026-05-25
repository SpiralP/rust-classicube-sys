use crate::{
    bindings::{BlockID, Inventory},
    std_types::c_int,
};

/// Gets the block at the nth index in the current hotbar.
///
/// # Panics
///
/// Panics if `Inventory.Offset + idx` is negative.
#[must_use]
pub fn Inventory_Get(idx: c_int) -> BlockID {
    let i = usize::try_from(unsafe { Inventory.Offset } + idx).expect("hotbar index out of range");
    unsafe { Inventory.Table[i] }
}

/// Sets the block at the nth index in the current hotbar.
///
/// # Panics
///
/// Panics if `Inventory.Offset + idx` is negative.
pub fn Inventory_Set(idx: c_int, block: BlockID) {
    let i = usize::try_from(unsafe { Inventory.Offset } + idx).expect("hotbar index out of range");
    unsafe {
        Inventory.Table[i] = block;
    }
}

/// Gets the currently selected block.
#[must_use]
pub fn Inventory_SelectedBlock() -> BlockID {
    unsafe { Inventory_Get(Inventory.SelectedIndex) }
}
