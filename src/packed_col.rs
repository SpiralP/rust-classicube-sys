use crate::bindings::*;

pub const fn PackedCol_R_Bits(col: u8) -> PackedCol {
    (col as PackedCol) << PACKEDCOL_R_SHIFT
}
pub const fn PackedCol_G_Bits(col: u8) -> PackedCol {
    (col as PackedCol) << PACKEDCOL_G_SHIFT
}
pub const fn PackedCol_B_Bits(col: u8) -> PackedCol {
    (col as PackedCol) << PACKEDCOL_B_SHIFT
}
pub const fn PackedCol_A_Bits(col: u8) -> PackedCol {
    (col as PackedCol) << PACKEDCOL_A_SHIFT
}

pub const fn PackedCol_Make(r: u8, g: u8, b: u8, a: u8) -> PackedCol {
    PackedCol_R_Bits(r) | PackedCol_G_Bits(g) | PackedCol_B_Bits(b) | PackedCol_A_Bits(a)
}

pub const PACKEDCOL_WHITE: PackedCol = PackedCol_Make(255, 255, 255, 255);
